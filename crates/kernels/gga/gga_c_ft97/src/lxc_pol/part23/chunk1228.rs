//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1228/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1228<F: Float>(t25: F, t30756: F, t202: F, t237: F, t5005: F, t22511: F, t33432: F, t3789: F, t1109: F, t1113: F, t24378: F, t27651: F, t30617: F, t108448: F, t109119: F, t109125: F, t109128: F, t109200: F, t1095: F, t123403: F, t123408: F, t123415: F, t123421: F, t123424: F, t123433: F, t13443: F, t13580: F, t17859: F, t17873: F, t17904: F, t18002: F, t18012: F, t218: F, t232: F, t24265: F, t25057: F, t27487: F, t27642: F, t27653: F, t27658: F, t27659: F, t27711: F, t30780: F, t35455: F, t3723: F, t3759: F, t3762: F, t5049: F, t6015: F, t684: F, t709: F) -> (F,) {
    let t123436 = t30756 * t25;
    let t123441 = t202 * t5005 * t237;
    let t123445 = t3789 * t33432 * t22511;
    let t123450 = t1109 * t1113;
    let t123456 = t27651 * t24378 * t30617;
    let t123458 = 0.77462893625097599762e-3 * t27487 * t17904 - 0.64507906339763927061e-5 * t27487 * t18012 - 0.44455354858818847408e-2 * t13443 * t25057 * t218 * t5049 * t709 + 0.474190451827401039e-1 * t13443 * t25057 * t18002 + 0.27039520901431665705e-3 * t3723 * t13580 * t109200 * t1095 + 0.13519760450715832853e-3 * t3723 * t123403 + t109119 + 0.85124811172839506172e-2 * t109125 + 0.28374937057613168724e-2 * t109128 - 0.60548059007656442387e-3 * t123408 * t108448 * t30780 * t684 + 0.60548059007656442388e-3 * t27658 * t123415 - 0.40365372671770961592e-3 * t27658 * t123421 - 0.44540303667943584666e-3 * t24265 * t232 * t123424 + 0.6809984893827160494e-1 * t27651 * t27642 * t27653 + 0.27568129967481981592e-3 * t123433 * t17873 - 0.23254900946437792e-1 * t3759 * t123436 * t3762 - 0.23254900946437792e-1 * t123441 * t6015 - 0.54493253106890798148e-2 * t123445 * t27659 * t35455 * t17859 - 0.47419045182740103901e-1 * t27711 * t25057 * t123450 * t709 - 0.85124811172839506173e-2 * t123456;
    (t123458,)
}
