//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1194/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1194(t10489: f64, t10627: f64, t10818: f64, t11054: f64, t11061: f64, t14365: f64, t1940: f64, t198: f64, t207: f64, t2070: f64, t2071: f64, t2394: f64, t2403: f64, t2408: f64, t2430: f64, t26581: f64, t26585: f64, t26590: f64, t2832: f64, t41161: f64, t4541: f64, t50066: f64, t51775: f64, t51792: f64, t51806: f64, t7428: f64, t7432: f64, t775: f64, t890: f64, t892: f64, t95527: f64, t95953: f64, t95964: f64, t95976: f64) -> f64 {
    let t96072 = t198 * t207 * t95953 * t892 + 3.0_f64 * t2403 * t2071 * t10489 - t1940 * t7432 * t11054 + 18.0_f64 * t2403 * t26590 * t50066 + 9.0_f64 * t2403 * t7428 * t2430 - 18.0_f64 * t4541 * t7432 * t51775 + 18.0_f64 * t4541 * t2071 * t10818 - 3.0_f64 * t1940 * t26585 * t2832 - 18.0_f64 * t2403 * t26585 * t14365 - 3.0_f64 * t1940 * t95527 * t890 - 9.0_f64 * t2403 * t7432 * t51806 - 9.0_f64 * t2403 * t7432 * t41161 + 9.0_f64 * t2403 * t26581 * t775 + 6.0_f64 * t198 * t10627 * t2070 * t892 + 18.0_f64 * t4541 * t7428 * t2394 - 6.0_f64 * t1940 * t95964 * t11061 + 6.0_f64 * t1940 * t26590 * t51792 + 6.0_f64 * t1940 * t95976 * t2408;
    t96072
}
