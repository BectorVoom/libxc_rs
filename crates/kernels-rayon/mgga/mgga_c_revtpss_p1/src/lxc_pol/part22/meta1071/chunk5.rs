//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3841/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3841(t3923: f64, t46478: f64, t13789: f64, t13791: f64, t13804: f64, t13926: f64, t1883: f64, t22046: f64, t22079: f64, t36776: f64, t3924: f64, t3934: f64, t3936: f64, t3938: f64, t48073: f64, t48105: f64, t48759: f64, t49146: f64, t5659: f64, t5673: f64, t6869: f64, t73818: f64, t73820: f64, t73837: f64, t73842: f64, t73847: f64, t73859: f64, t9810: f64) -> (f64, f64) {
    let t73861 = t46478 * t3923;
    let t73870 = -0.80031500487063509014e-2_f64 * t73818 + 0.10289764348336736874e-1_f64 * t13804 * t13789 * t73820 * t13791 - 0.51448821741683684367e-2_f64 * t13804 * t36776 * t48105 * t49146 + 0.17149607247227894789e-2_f64 * t3934 * t13789 * t48073 * t6869 - 0.85748036236139473944e-3_f64 * t3934 * t36776 * t13926 * t5659 + 0.34299214494455789578e-2_f64 * t3934 * t13789 * t1883 * t73837 + 0.17149607247227894789e-2_f64 * t3934 * t13789 * t73842 * t3938 + 0.17149607247227894789e-2_f64 * t3934 * t3936 * t73847 * t3938 + 0.85748036236139473944e-3_f64 * t3934 * t3936 * t22079 * t9810 + 0.10164000561857065645e-3_f64 * t73859 + 0.51448821741683684368e-2_f64 * t48759 * t5673 * t22046 * t73861 - 0.21437009059034868486e-3_f64 * t3934 * t5673 * t22046 * t3924;
    (t73861, t73870)
}
