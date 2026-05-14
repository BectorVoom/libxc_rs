//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1182/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1182<F: Float>(t173: F, t22583: F, t25694: F, t423: F, t100542: F, t100558: F, t101145: F, t101173: F, t101193: F, t101200: F, t101201: F, t101209: F, t101228: F, t101234: F, t11095: F, t11098: F, t11104: F, t1603: F, t1614: F, t1624: F, t1630: F, t1632: F, t1647: F, t22563: F, t22585: F, t22605: F, t22613: F, t22718: F, t22834: F, t25658: F, t25692: F, t25695: F, t25784: F, t3019: F, t34434: F, t37481: F, t5513: F, t5522: F, t5589: F, t73: F, t7837: F, t7857: F, t920: F, t92353: F, t92354: F, t92463: F, t92864: F, t92896: F, t92969: F, t92975: F, t930: F, t938: F) -> (F,) {
    let t101243 = 0.49489226297715094073e-4 * t22583 * t173 * t423 * t25694;
    let t101244 = -0.49489226297715094073e-4 * t101173 + 0.1836608226397146721e-4 * t7837 * t22563 * t7857 * t11098 + 0.27568129967481981592e-3 * t7837 * t5522 * t938 * t1614 * t11095 + 0.27568129967481981592e-3 * t92864 * t11104 + 0.55136259934963963185e-4 * t7837 * t22563 * t1614 * t1630 * t11098 + 0.17659850543899795696e-2 * t92969 + 0.12768721675925925926e-1 * t92975 + 0.26724182200766150799e-2 * t101193 * t34434 * t22605 - 0.1758835445293583825e-6 * t1624 * t92354 * t37481 * t5589 * t101200 * t101201 - 0.59346127734643676855e-4 * t92353 * t92896 * t101200 * t100558 + 0.59346127734643676855e-4 * t101209 * t101200 * t100542 - 0.14846767889314528222e-4 * t22583 * t25692 * t920 * t1630 * t1632 - 0.14846767889314528222e-3 * t22583 * t22585 * t930 * t1647 - 0.23254900946437792e-1 * t1603 * t5513 * t101145 - 0.46509801892875584e-1 * t22834 * t25784 + 0.27039520901431665706e-3 * t3019 * t101228 - 0.46509801892875584e-1 * t1603 * t22718 * t25658 + 0.89080607335887169332e-3 * t22613 * t73 * t101234 - 0.3959138103817207526e-3 * t92463 * t25695 + t101243;
    (t101244,)
}
