//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 854/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk854<F: Float>(t44643: F, t44395: F, t987: F, t11271: F, t2268: F, t6763: F, t1063: F, t11254: F, t2343: F, t6519: F, t42898: F, t2787: F, t31557: F, t32067: F, t42885: F, t42889: F, t44618: F, t44622: F, t44624: F, t44627: F, t44630: F, t44633: F, t44635: F, t44638: F, t44642: F, t535: F, t999: F) -> F {
    let t44644 = F::new(0.31616674039640166221e-2) * t44643;
    let t44652 = t987 * t44395;
    let t44658 = F::new(0.19918504644973304719e0) * t2268 * t11271 * t6763;
    let t44662 = F::new(0.56910013271352299198e-1) * t1063 * t2343 * t11254 * t6519;
    let t44665 = F::new(0.47425011059460249332e-2) * t42898;
    let t44666 = t44618 + t44622 + t44624 + t44627 + t44630 - t44633 - t44635 - t44638 + t44642 - t44644 - F::new(0.1138200265427045984e0) * t1063 * t2343 * t2787 * t31557 - F::new(0.56910013271352299198e-1) * t1063 * t999 * t32067 + F::new(0.56910013271352299198e-1) * t2268 * t535 * t44652 - t44658 - t44662 - F::new(0.47425011059460249332e-2) * t42885 - F::new(0.47425011059460249332e-2) * t42889 + t44665;
    t44666
}
