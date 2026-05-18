//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1075/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1075<F: Float>(t45163: F, t45168: F, t45170: F, t45175: F, t45179: F, t45183: F, t45187: F, t45189: F, t45193: F, t45197: F, t45201: F, t45205: F, t45207: F, t45209: F, t45212: F, t45215: F, t45217: F, t45219: F) -> F {
    let t48376 = -F::new(0.85129199786595678799e-5) * t45163 + F::new(0.2727466165424534173e-1) * t45168 + F::new(0.1702583995731913576e-4) * t45170 - F::new(0.212822999466489197e-4) * t45175 - F::new(0.10215503974391481456e-3) * t45179 + F::new(0.15323255961587222184e-3) * t45183 + F::new(0.5107751987195740728e-4) * t45187 + F::new(0.1702583995731913576e-4) * t45189 + F::new(0.1702583995731913576e-4) * t45193 + F::new(0.638468998399467591e-4) * t45197 - F::new(0.5107751987195740728e-4) * t45201 - F::new(0.1702583995731913576e-4) * t45205 - F::new(0.13637330827122670865e-1) * t45207 + F::new(0.35922725105591425692e0) * t45209 + F::new(0.35922725105591425692e0) * t45212 + F::new(0.35922725105591425692e0) * t45215 + F::new(0.11974241701863808564e0) * t45217 + F::new(0.11974241701863808564e0) * t45219;
    t48376
}
