//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1295/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1295(t25705: f64, t3336: f64, t11108: f64, t7177: f64, t1989: f64, t41937: f64, t10489: f64, t10627: f64, t10818: f64, t11054: f64, t11061: f64, t14365: f64, t1940: f64, t1962: f64, t1963: f64, t198: f64, t207: f64, t2394: f64, t2403: f64, t2408: f64, t2430: f64, t25436: f64, t25440: f64, t25445: f64, t2832: f64, t41161: f64, t4541: f64, t50066: f64, t51775: f64, t51792: f64, t51806: f64, t7087: f64, t7091: f64, t775: f64, t890: f64, t892: f64, t92742: f64, t92775: f64, t93396: f64, t93404: f64) -> (f64, f64, f64, f64) {
    let t94138 = t25705 * t3336;
    let t94142 = t7177 * t11108;
    let t94149 = t1989 * t41937;
    let t94213 = -18.0_f64 * t2403 * t25440 * t14365 - 3.0_f64 * t1940 * t92775 * t890 + t198 * t207 * t93396 * t892 + 18.0_f64 * t2403 * t25445 * t50066 - 9.0_f64 * t2403 * t7091 * t51806 - 9.0_f64 * t2403 * t7091 * t41161 - t1940 * t7091 * t11054 - 3.0_f64 * t1940 * t25440 * t2832 + 18.0_f64 * t4541 * t1963 * t10818 + 3.0_f64 * t2403 * t1963 * t10489 - 18.0_f64 * t4541 * t7091 * t51775 + 6.0_f64 * t198 * t10627 * t1962 * t892 + 18.0_f64 * t4541 * t7087 * t2394 + 9.0_f64 * t2403 * t25436 * t775 - 6.0_f64 * t1940 * t92742 * t11061 + 9.0_f64 * t2403 * t7087 * t2430 + 6.0_f64 * t1940 * t93404 * t2408 + 6.0_f64 * t1940 * t25445 * t51792;
    (t94138, t94142, t94149, t94213)
}
