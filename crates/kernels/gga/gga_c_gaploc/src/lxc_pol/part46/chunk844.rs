//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 844/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk844<F: Float>(t13154: F, t24661: F, t13096: F, t2089: F, t13153: F, t3251: F, t4752: F, t13023: F, t2103: F, t4673: F, t1445: F, t43213: F, t833: F, t1457: F, t2004: F, t2087: F, t42993: F, t43307: F, t43588: F, t43592: F, t43597: F, t43601: F, t43602: F, t43603: F, t43604: F, t43605: F, t43606: F, t43607: F, t43609: F, t43611: F, t43617: F, t723: F) -> (F,) {
    let t43619 = 0.42900587942220512003e1 * t24661 * t13154;
    let t43620 = t2089 * t13096;
    let t43627 = 0.28600391961480341335e1 * t13153 * t4752 * t3251;
    let t43630 = 0.47667319935800568892e0 * t2103 * t4673 * t13023;
    let t43636 = 0.11502877786176224903e2 * t833 * t1445 * t43213;
    let t43637 = -0.14300195980740170668e1 * t43588 + t43592 - t43597 + t43601 + t43602 - t43603 - t43604 - t43605 + t43606 - t43607 + 0.38342925953920749676e0 * t43609 + 0.38342925953920749676e0 * t43611 + 0.35750489951850426669e0 * t2004 * t1457 * t42993 + t43617 + t43619 - 0.69017266717057349418e1 * t2087 * t1445 * t43620 * t723 + t43627 + t43630 + 0.71500979903700853338e0 * t2103 * t1457 * t43307 + t43636;
    (t43637,)
}
