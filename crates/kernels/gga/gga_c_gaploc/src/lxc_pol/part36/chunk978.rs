//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 978/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk978<F: Float>(t13069: F, t7416: F, t13154: F, t24799: F, t24661: F, t13096: F, t2089: F, t13153: F, t3251: F, t4752: F, t13023: F, t2103: F, t4673: F) -> (F, F, F, F, F, F) {
    let t43611 = t7416 * t13069;
    let t43617 = F::new(0.42900587942220512003e1) * t24799 * t13154;
    let t43619 = F::new(0.42900587942220512003e1) * t24661 * t13154;
    let t43620 = t2089 * t13096;
    let t43627 = F::new(0.28600391961480341335e1) * t13153 * t4752 * t3251;
    let t43630 = F::new(0.47667319935800568892e0) * t2103 * t4673 * t13023;
    (t43611, t43617, t43619, t43620, t43627, t43630)
}
