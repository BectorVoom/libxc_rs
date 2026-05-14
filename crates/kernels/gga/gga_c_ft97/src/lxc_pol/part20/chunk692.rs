//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 692/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk692<F: Float>(t14895: F, t14902: F, t1775: F, t4203: F, t4207: F, t4200: F, t13309: F, t4199: F, t10580: F, t2: F, t13315: F, t13320: F, t14624: F, t2771: F, t13352: F, t10603: F, t14671: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14949 = 4.0 / 9.0 * t14895;
    let t14951 = 2.0 / 3.0 * t14902;
    let t14953 = 2.0 / 9.0 * t1775 * t4203;
    let t14955 = 4.0 / 9.0 * t1775 * t4207;
    let t14957 = 4.0 / 27.0 * t1775 * t4200;
    let t14958 = t4199 * t13309;
    let t14961 = t10580 * t2;
    let t14962 = t14961 * t13315;
    let t14965 = t4199 * t13320;
    let t14968 = t2771 * t14624;
    let t14971 = t4199 * t13352;
    let t14974 = t10603 * t14671;
    (t14949, t14951, t14953, t14955, t14957, t14958, t14962, t14965, t14968, t14971, t14974)
}
