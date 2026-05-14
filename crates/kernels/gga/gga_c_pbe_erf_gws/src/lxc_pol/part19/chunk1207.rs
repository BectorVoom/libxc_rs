//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1207/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1207<F: Float>(t55547: F, t55548: F, t57060: F, t57062: F, t57064: F, t57066: F, t57068: F, t57070: F, t57073: F, t57075: F, t57077: F, t57079: F, t55556: F, t55557: F, t57082: F, t57086: F, t57088: F, t57090: F, t57092: F, t57094: F, t57096: F, t57098: F, t57100: F, t57102: F, t57104: F) -> (F, F) {
    let t58697 = -t57060 / 12.0 - t57062 / 96.0 - t57064 / 24.0 + t57066 / 48.0 + 7.0 / 144.0 * t57068 + t57070 / 96.0 + t55547 - t57073 / 48.0 - t57075 / 96.0 - t57077 / 12.0 - t55548 - t57079 / 48.0;
    let t58709 = -t57082 / 384.0 + t57086 / 24.0 - t57088 / 12.0 - t57090 / 48.0 - t57092 / 384.0 - 5.0 / 96.0 * t57094 + t57096 / 48.0 + t57098 / 24.0 + t55556 + t57100 / 48.0 - t57102 / 48.0 - t55557 + 7.0 / 576.0 * t57104;
    (t58697, t58709)
}
