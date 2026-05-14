//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 933/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk933<F: Float>(t16538: F, t587: F, t2368: F, t4703: F, t4706: F, t4787: F, t6874: F, t6880: F, t12076: F, t6879: F, t4761: F, t1980: F, t10543: F, t16361: F, t16363: F, t16441: F, t16459: F, t16462: F, t16519: F, t1674: F, t1686: F, t2396: F, t45: F, t4757: F, t4764: F, t4783: F, t6851: F, t6857: F, t6876: F, t6881: F) -> (F, F, F, F, F, F) {
    let t16540 = 0.62182e-1 * t16538 * t587;
    let t16541 = t2368 * t4703;
    let t16543 = 2.0 * t16541 * t4706;
    let t16546 = t4787 * t6874;
    let t16547 = t16546 * t6880;
    let t16550 = t6879 * t12076;
    let t16555 = t4761 * t6874;
    let t16556 = t16555 * t1980;
    let t16561 = t16361 + t16363 + t16441 + 0.19751789702565206229e-1 * t45 * t16459 - 0.11696446794910408142e1 * t16462 * t1686 - 0.58482233974552040708e0 * t6851 * t4783 - 0.11696446794910408142e1 * t4757 * t6876 + 0.11696446794910408142e1 * t6851 * t4764 - 0.58482233974552040708e0 * t1674 * t16519 - t16540 - t16543 - 0.58482233974552040708e0 * t10543 * t2396 - 0.34631511798751726598e2 * t1674 * t16547 - 0.17315755899375863299e2 * t1674 * t16550 + 0.23392893589820816284e1 * t4757 * t6857 + 0.23392893589820816284e1 * t1674 * t16556 - 0.34631511798751726598e2 * t4757 * t6881;
    (t16540, t16543, t16547, t16550, t16556, t16561)
}
