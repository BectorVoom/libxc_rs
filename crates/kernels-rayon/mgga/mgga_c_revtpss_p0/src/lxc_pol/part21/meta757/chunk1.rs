//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2656/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2656(t48876: f64, t40763: f64, t5609: f64, t9793: f64, t13830: f64, t9775: f64, t13826: f64, t3989: f64, t1410: f64, t3829: f64, t3934: f64, t46403: f64, t48833: f64, t48837: f64, t48845: f64, t48848: f64, t48849: f64, t48851: f64, t48853: f64, t48855: f64, t48865: f64, t48869: f64, t48872: f64, t5591: f64, t5671: f64, t5673: f64, t5674: f64, t828: f64, t9899: f64, t9942: f64) -> f64 {
    let t48877 = 0.15246000842785598467e-4_f64 * t48876;
    let t48879 = t9793 * t40763 * t5609;
    let t48881 = t9775 * t13830;
    let t48888 = t3989 * t13826;
    let t48890 = 0.28900264064772933812e-2_f64 * t48833 + 0.18007087609589289528e-1_f64 * t48837 + 0.42874018118069736972e-3_f64 * t5671 * t5673 * t5674 * t46403 + 0.25724410870841842183e-2_f64 * t48845 - t48848 - 0.51384669507166276316e-2_f64 * t48849 - 0.68026775414003982662e-1_f64 * t48851 + 0.72250660161932334527e-3_f64 * t48853 + 0.60023625365297631762e-1_f64 * t48855 - 0.21437009059034868486e-3_f64 * t3934 * t5673 * t5674 * t9899 - 0.17149607247227894789e-3_f64 * t48865 + t48869 + 0.27107389498472794076e-3_f64 * t48872 - t48877 + 0.81322168495418382223e-4_f64 * t48879 + 0.45732285992607719437e-3_f64 * t48881 - 0.77173232612525526549e-1_f64 * t1410 * t9942 * t828 * t5591 * t3829 + 0.36014175219178579057e0_f64 * t48888;
    t48890
}
