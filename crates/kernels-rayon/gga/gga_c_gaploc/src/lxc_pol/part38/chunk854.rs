//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 854/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk854(t44643: f64, t44395: f64, t987: f64, t11271: f64, t2268: f64, t6763: f64, t1063: f64, t11254: f64, t2343: f64, t6519: f64, t42898: f64, t2787: f64, t31557: f64, t32067: f64, t42885: f64, t42889: f64, t44618: f64, t44622: f64, t44624: f64, t44627: f64, t44630: f64, t44633: f64, t44635: f64, t44638: f64, t44642: f64, t535: f64, t999: f64) -> f64 {
    let t44644 = 0.31616674039640166221e-2_f64 * t44643;
    let t44652 = t987 * t44395;
    let t44658 = 0.19918504644973304719e0_f64 * t2268 * t11271 * t6763;
    let t44662 = 0.56910013271352299198e-1_f64 * t1063 * t2343 * t11254 * t6519;
    let t44665 = 0.47425011059460249332e-2_f64 * t42898;
    let t44666 = t44618 + t44622 + t44624 + t44627 + t44630 - t44633 - t44635 - t44638 + t44642 - t44644 - 0.1138200265427045984e0_f64 * t1063 * t2343 * t2787 * t31557 - 0.56910013271352299198e-1_f64 * t1063 * t999 * t32067 + 0.56910013271352299198e-1_f64 * t2268 * t535 * t44652 - t44658 - t44662 - 0.47425011059460249332e-2_f64 * t42885 - 0.47425011059460249332e-2_f64 * t42889 + t44665;
    t44666
}
