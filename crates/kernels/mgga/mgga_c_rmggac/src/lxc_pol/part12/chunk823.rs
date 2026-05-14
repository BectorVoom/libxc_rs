//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 823/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk823<F: Float>(t1540: F, t2144: F, t2147: F, t5055: F, t7524: F, t36895: F, t8571: F, t35535: F, t36450: F, t8443: F, t36734: F, t1475: F, t1970: F, t1971: F, t875: F, t876: F) -> (F, F, F, F, F, F, F) {
    let t39953 = t1540 * t2144;
    let t39954 = t39953 * t2147;
    let t39956 = t5055 * t7524;
    let t39964 = t8571 * t36895;
    let t39966 = t8571 * t35535;
    let t39968 = t36450 * t8443;
    let t39970 = t36734 * t8443;
    let t39971 = 0.19863479950205658386e-4 * t39970;
    let t39975 = t1970 * t1971 * t875 * t1475 * t876;
    (t39954, t39956, t39964, t39966, t39968, t39971, t39975)
}
