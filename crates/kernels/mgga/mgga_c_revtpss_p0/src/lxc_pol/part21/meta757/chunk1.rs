//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2656/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2656<F: Float>(t48876: F, t40763: F, t5609: F, t9793: F, t13830: F, t9775: F, t13826: F, t3989: F, t1410: F, t3829: F, t3934: F, t46403: F, t48833: F, t48837: F, t48845: F, t48848: F, t48849: F, t48851: F, t48853: F, t48855: F, t48865: F, t48869: F, t48872: F, t5591: F, t5671: F, t5673: F, t5674: F, t828: F, t9899: F, t9942: F) -> F {
    let t48877 = F::cast_from(0.15246000842785598467e-4_f64) * t48876;
    let t48879 = t9793 * t40763 * t5609;
    let t48881 = t9775 * t13830;
    let t48888 = t3989 * t13826;
    let t48890 = F::cast_from(0.28900264064772933812e-2_f64) * t48833 + F::cast_from(0.18007087609589289528e-1_f64) * t48837 + F::cast_from(0.42874018118069736972e-3_f64) * t5671 * t5673 * t5674 * t46403 + F::cast_from(0.25724410870841842183e-2_f64) * t48845 - t48848 - F::cast_from(0.51384669507166276316e-2_f64) * t48849 - F::cast_from(0.68026775414003982662e-1_f64) * t48851 + F::cast_from(0.72250660161932334527e-3_f64) * t48853 + F::cast_from(0.60023625365297631762e-1_f64) * t48855 - F::cast_from(0.21437009059034868486e-3_f64) * t3934 * t5673 * t5674 * t9899 - F::cast_from(0.17149607247227894789e-3_f64) * t48865 + t48869 + F::cast_from(0.27107389498472794076e-3_f64) * t48872 - t48877 + F::cast_from(0.81322168495418382223e-4_f64) * t48879 + F::cast_from(0.45732285992607719437e-3_f64) * t48881 - F::cast_from(0.77173232612525526549e-1_f64) * t1410 * t9942 * t828 * t5591 * t3829 + F::cast_from(0.36014175219178579057e0_f64) * t48888;
    t48890
}
