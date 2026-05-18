//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1073/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1073<F: Float>(t35705: F, t37815: F, t37816: F, t37818: F, t40360: F, t40362: F, t40365: F, t40367: F, t40372: F, t40377: F, t40379: F, t40384: F, t40389: F, t40391: F, t40396: F, t40401: F, t40403: F, t40414: F) -> F {
    let t43410 = -F::new(0.212822999466489197e-4) * t40360 - F::new(0.1064114997332445985e-4) * t40362 - F::new(0.85129199786595678799e-5) * t40365 - F::new(0.1702583995731913576e-4) * t40367 - F::new(0.638468998399467591e-4) * t40372 - F::new(0.3192344991997337955e-4) * t40377 + F::new(0.638468998399467591e-4) * t40379 + F::new(0.638468998399467591e-4) * t40384 + F::new(0.3192344991997337955e-4) * t40389 + F::new(0.212822999466489197e-4) * t40391 + F::new(0.212822999466489197e-4) * t40396 + F::new(0.1064114997332445985e-4) * t40401 - F::new(0.212822999466489197e-4) * t40403 - t37815 - t37816 - t37818 - F::new(0.14088275218353950416e-1) * t35705 + F::new(0.212822999466489197e-4) * t40414;
    t43410
}
