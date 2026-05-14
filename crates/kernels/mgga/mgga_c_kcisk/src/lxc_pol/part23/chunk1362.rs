//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1362/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1362<F: Float>(t110244: F, t110341: F, t110423: F, t113788: F, t113875: F, t113879: F, t113941: F, t113947: F, t113951: F, t113955: F, t113959: F, t113963: F, t113973: F, t32010: F, t32035: F, t32066: F, t32087: F, t33428: F, t33439: F, t9426: F, t9805: F, t9809: F) -> (F,) {
    let t113982 = 0.69444444444444444446e-2 * t113941 * t32010 + 0.26805555555555555556e-2 * t113947 * t32010 + t113951 + 0.10185185185185185186e0 * t110244 * t9809 - t113955 - 0.55555555555555555558e-1 * t110341 * t9809 - 0.23280625000000000001e-2 * t113959 * t32035 + 0.23148148148148148148e-2 * t113963 + 0.69444444444444444446e-2 * t110423 * t33428 + 0.34722222222222222223e-2 * t32087 * t113875 + 0.46296296296296296297e-2 * t32087 * t113879 + 0.41666666666666666668e-1 * t32087 * t113973 - 0.33950617283950617285e-1 * t110244 * t9805 - 0.8041666666666666667e-2 * t32066 * t33439 - 0.24125000000000000001e-1 * t9426 * t113788;
    (t113982,)
}
