//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 951/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk951<F: Float>(t1017: F, t525: F, t1181: F, t2068: F, t7351: F, t4773: F, t570: F, t30661: F, t30664: F, t30670: F, t30672: F, t30675: F, t30690: F, t30695: F, t30705: F, t30709: F, t34655: F, t34657: F, t34660: F, t34663: F, t34667: F, t34671: F, t34675: F) -> (F, F) {
    let t34681 = t525 * t1017;
    let t34684 = t2068 * t1181 * t7351 * t34681;
    let t34686 = t570 * t4773;
    let t34688 = 0.40015750243531754508e-2 * t30661 - t30664 - t30670 + t30672 - t34655 - 0.17149607247227894789e-2 * t30675 - t34657 / 96.0 + t34660 + 0.31448092289604152068e-3 * t34663 + 0.64311027177104605458e-3 * t34667 + 0.47172138434406228102e-2 * t34671 + 0.41930789719472202758e-3 * t34675 - 0.34299214494455789578e-2 * t30690 + 0.7145669686344956162e-3 * t30695 - 0.10482697429868050689e-2 * t30705 - 0.62896184579208304134e-3 * t30709 - 0.64311027177104605458e-3 * t34684 - t34686 / 48.0;
    (t34681, t34688)
}
