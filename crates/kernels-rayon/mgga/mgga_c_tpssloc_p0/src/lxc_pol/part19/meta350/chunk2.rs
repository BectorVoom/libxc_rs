//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1274/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1274(t324: f64, t41749: f64, t41762: f64, t10603: f64, t2932: f64, t10717: f64, t10720: f64, t10724: f64, t10734: f64, t10740: f64, t10747: f64, t10753: f64, t10756: f64, t10765: f64, t10771: f64, t10825: f64, t10828: f64, t14259: f64, t2880: f64, t2889: f64, t2905: f64, t2924: f64, t2930: f64, t2933: f64, t41620: f64, t41622: f64, t41625: f64, t41627: f64, t41635: f64, t41639: f64, t41722: f64, t950: f64) -> (f64, f64) {
    let t41764 = (t41749 + t41762) * t324;
    let t41769 = t10603 * t2932;
    let t41790 = -0.19751673498613801407e-1_f64 * t41764 - t41620 - t41622 - t41625 - t41627 - t41635 - t41639 + t41722 - 0.46785788981077169656e1_f64 * t2905 * t10753 * t950 + 0.69263436422725855036e2_f64 * t2930 * t41769 * t950 + 0.61524113149298439947e4_f64 * t10756 * t14259 * t2924 - 0.14035736694323150897e2_f64 * t10747 * t10720 + 0.20779030926817756511e3_f64 * t10825 * t10724 - 0.62337092780453269531e3_f64 * t10828 * t2933 * t2924 - 24.0_f64 * t10740 * t10734 + 0.3859675079686208416e3_f64 * t10765 * t10717 - 0.11579025239058625248e4_f64 * t10771 * t2889 * t2880;
    (t41764, t41790)
}
