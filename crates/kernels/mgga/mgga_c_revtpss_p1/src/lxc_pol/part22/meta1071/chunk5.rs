//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3841/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3841<F: Float>(t3923: F, t46478: F, t13789: F, t13791: F, t13804: F, t13926: F, t1883: F, t22046: F, t22079: F, t36776: F, t3924: F, t3934: F, t3936: F, t3938: F, t48073: F, t48105: F, t48759: F, t49146: F, t5659: F, t5673: F, t6869: F, t73818: F, t73820: F, t73837: F, t73842: F, t73847: F, t73859: F, t9810: F) -> (F, F) {
    let t73861 = t46478 * t3923;
    let t73870 = -F::cast_from(0.80031500487063509014e-2_f64) * t73818 + F::cast_from(0.10289764348336736874e-1_f64) * t13804 * t13789 * t73820 * t13791 - F::cast_from(0.51448821741683684367e-2_f64) * t13804 * t36776 * t48105 * t49146 + F::cast_from(0.17149607247227894789e-2_f64) * t3934 * t13789 * t48073 * t6869 - F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t36776 * t13926 * t5659 + F::cast_from(0.34299214494455789578e-2_f64) * t3934 * t13789 * t1883 * t73837 + F::cast_from(0.17149607247227894789e-2_f64) * t3934 * t13789 * t73842 * t3938 + F::cast_from(0.17149607247227894789e-2_f64) * t3934 * t3936 * t73847 * t3938 + F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t3936 * t22079 * t9810 + F::cast_from(0.10164000561857065645e-3_f64) * t73859 + F::cast_from(0.51448821741683684368e-2_f64) * t48759 * t5673 * t22046 * t73861 - F::cast_from(0.21437009059034868486e-3_f64) * t3934 * t5673 * t22046 * t3924;
    (t73861, t73870)
}
