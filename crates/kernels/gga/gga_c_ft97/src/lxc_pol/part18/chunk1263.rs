//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1263/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1263<F: Float>(t103: F, t25846: F, t101: F, t26373: F, t1882: F, t26262: F, t1825: F, t26113: F, t26142: F, t100345: F, t11855: F, t11859: F, t11863: F, t1643: F, t1901: F, t1902: F, t22940: F, t26357: F, t26423: F, t3271: F, t379: F, t39167: F, t446: F, t452: F, t47659: F, t6478: F, t6538: F, t83: F, t8360: F, t8372: F, t91539: F, t93815: F, t93817: F, t93819: F) -> (F, F) {
    let t103864 = t103 * t25846;
    let t103872 = t101 * t26373;
    let t103881 = 4.0 / 9.0 * t1882 * t26262;
    let t103892 = t1825 * t26113;
    let t103905 = 2.0 / 9.0 * t1882 * t26142;
    let t103906 = 2.0 / 9.0 * t1901 * t8372 * t26357 + 2.0 / 9.0 * t1901 * t1902 * t103864 * t379 + 4.0 / 9.0 * t47659 * t91539 * t11859 + 4.0 / 3.0 * t47659 * t103872 * t11855 + 2.0 / 3.0 * t446 * t452 * t22940 * t3271 - t103881 + t93815 / 9.0 + 2.0 / 9.0 * t93817 + 2.0 / 9.0 * t93819 - 2.0 / 27.0 * t1901 * t39167 * t6538 * t1643 - 2.0 / 9.0 * t1901 * t11863 * t100345 - 2.0 / 3.0 * t446 * t83 * t103892 + t446 * t452 * t8360 * t6478 / 3.0 + 2.0 / 3.0 * t446 * t452 * t1825 * t26423 - t103905;
    (t103892, t103906)
}
