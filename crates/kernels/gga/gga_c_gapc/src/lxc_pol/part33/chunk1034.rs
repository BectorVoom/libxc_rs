//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1034/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1034<F: Float>(t20171: F, t33287: F, t5708: F, t19533: F, t19535: F, t34742: F, t34745: F, t34747: F, t34749: F, t34752: F, t34755: F, t34757: F, t34759: F, t34761: F, t11587: F, t11591: F, t3060: F) -> (F, F) {
    let t34764 = t5708 * t33287 * t20171;
    let t34767 = t19533 * t33287 * t19535;
    let t34769 = -0.49166375783284505216e-7 * t34742 - 0.67530371184977617164e-6 * t34745 - 0.50595483470764842601e-7 * t34747 - 0.77294542590142724635e-6 * t34749 + 0.40483072916666666668e-4 * t34752 + 0.20241536458333333334e-3 * t34755 - 0.2318836277704281739e-4 * t34757 + 0.32827263770475230566e-8 * t34759 - 0.34842871069624090849e-4 * t34761 - 0.31675337336021900772e-5 * t34764 - 0.31675337336021900772e-5 * t34767;
    let t34772 = t3060 * t11587 * t11591;
    (t34769, t34772)
}
