//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1159/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1159<F: Float>(t25705: F, t3336: F, t11108: F, t7177: F, t1989: F, t41937: F, t10489: F, t10627: F, t10818: F, t11054: F, t11061: F, t14365: F, t1940: F, t1962: F, t1963: F, t198: F, t207: F, t2394: F, t2403: F, t2408: F, t2430: F, t25436: F, t25440: F, t25445: F, t2832: F, t41161: F, t4541: F, t50066: F, t51775: F, t51792: F, t51806: F, t7087: F, t7091: F, t775: F, t890: F, t892: F, t92742: F, t92775: F, t93396: F, t93404: F) -> (F, F, F, F) {
    let t94138 = t25705 * t3336;
    let t94142 = t7177 * t11108;
    let t94149 = t1989 * t41937;
    let t94213 = -18.0 * t2403 * t25440 * t14365 - 3.0 * t1940 * t92775 * t890 + t198 * t207 * t93396 * t892 + 18.0 * t2403 * t25445 * t50066 - 9.0 * t2403 * t7091 * t51806 - 9.0 * t2403 * t7091 * t41161 - t1940 * t7091 * t11054 - 3.0 * t1940 * t25440 * t2832 + 18.0 * t4541 * t1963 * t10818 + 3.0 * t2403 * t1963 * t10489 - 18.0 * t4541 * t7091 * t51775 + 6.0 * t198 * t10627 * t1962 * t892 + 18.0 * t4541 * t7087 * t2394 + 9.0 * t2403 * t25436 * t775 - 6.0 * t1940 * t92742 * t11061 + 9.0 * t2403 * t7087 * t2430 + 6.0 * t1940 * t93404 * t2408 + 6.0 * t1940 * t25445 * t51792;
    (t94138, t94142, t94149, t94213)
}
