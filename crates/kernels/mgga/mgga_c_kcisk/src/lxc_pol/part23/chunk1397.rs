//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1397/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1397<F: Float>(t113607: F, t113644: F, t113686: F, t113732: F, t113774: F, t113811: F, t113859: F, t113904: F, t113944: F, t113982: F, t114025: F, t114064: F, t114103: F, t114142: F, t114176: F, t114217: F, t114258: F, t114298: F, t114339: F, t114365: F, t114403: F, t114441: F, t114479: F, t114512: F, t114543: F, t114583: F, t114627: F, t114669: F, t114698: F, t114740: F, t114776: F, t114805: F, t504: F) -> (F,) {
    let t114811 = (t113944 + t113904 + t113859 + t113811 + t113774 + t113732 + t113686 + t113644 + t113607 + t114176 + t114142 + t114103 + t114064 + t114025 + t113982 + t114805 + t114776 + t114740 + t114698 + t114669 + t114627 + t114583 + t114543 + t114512 + t114479 + t114441 + t114403 + t114365 + t114217 + t114339 + t114298 + t114258) * t504;
    (t114811,)
}
