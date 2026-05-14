//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1081/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1081<F: Float>(t12061: F, t9108: F, t1973: F, t1965: F, t9103: F, t12018: F, t12059: F, t12105: F, t18546: F, t18643: F, t1975: F, t24748: F, t24751: F, t24755: F, t24762: F, t24765: F, t24768: F, t24771: F, t24775: F, t24778: F, t5373: F, t5398: F, t5415: F, t7472: F, t7494: F) -> (F,) {
    let t24781 = t9108 * t12061;
    let t24782 = t24781 * t1973;
    let t24785 = t9103 * t1965;
    let t24788 = 0.17315755899375863299e2 * t5415 * t24748 + 0.34631511798751726598e2 * t5415 * t24751 + 0.1025389702100779493e4 * t12105 * t24755 - 4.0 * t18643 * t7472 + 0.64329366355741395948e2 * t18546 * t7494 + 6.0 * t5398 * t24762 - 4.0 * t5373 * t24765 - 0.19298809906722418785e3 * t12018 * t24768 - 2.0 * t5373 * t24771 + 0.32164683177870697974e2 * t5398 * t24775 + 0.64329366355741395948e2 * t5398 * t24778 + 0.20691336878655965246e4 * t12059 * t24782 + 1.0 * t24785 * t1975;
    (t24788,)
}
