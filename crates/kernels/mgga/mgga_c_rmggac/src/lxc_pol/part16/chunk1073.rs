//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1073/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1073<F: Float>(t10252: F, t38965: F, t42788: F, t42794: F, t45055: F, t45060: F, t45062: F, t45064: F, t45069: F, t45074: F, t45080: F, t45087: F, t45089: F, t45091: F, t45094: F, t45099: F, t45104: F, t45109: F, t5016: F) -> F {
    let t48342 = -F::cast_from(0.5107751987195740728e-4_f64) * t45055 + F::cast_from(0.1915406995198402773e-3_f64) * t45060 - F::cast_from(0.638468998399467591e-4_f64) * t45062 - F::cast_from(0.5107751987195740728e-4_f64) * t45064 + F::cast_from(0.85129199786595678799e-5_f64) * t45069 - F::cast_from(0.2553875993597870364e-4_f64) * t45074 + F::cast_from(0.2553875993597870364e-4_f64) * t45080 - F::cast_from(0.11974241701863808564e0_f64) * t5016 * t10252 + t42788 + F::cast_from(0.20455996240684006298e-1_f64) * t45087 - F::cast_from(0.23836175940246790064e-3_f64) * t45089 - F::cast_from(0.13242319966803772257e-3_f64) * t38965 + t42794 - F::cast_from(0.85129199786595678799e-5_f64) * t45091 - F::cast_from(0.85129199786595678799e-5_f64) * t45094 + F::cast_from(0.1702583995731913576e-4_f64) * t45099 + F::cast_from(0.1702583995731913576e-4_f64) * t45104 + F::cast_from(0.85129199786595678799e-5_f64) * t45109;
    t48342
}
