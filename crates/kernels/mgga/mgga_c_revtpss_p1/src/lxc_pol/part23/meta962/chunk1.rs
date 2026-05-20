//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3251/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3251<F: Float>(t13847: F, t1883: F, t73856: F, t9816: F, t22895: F, t9962: F, t125: F, t22813: F, t22857: F, t13783: F, t1399: F, t22046: F, t22079: F, t3934: F, t3936: F, t3938: F, t4003: F, t47248: F, t5591: F, t5627: F, t5659: F, t5671: F, t5673: F, t6862: F, t73726: F, t73729: F, t73734: F, t73738: F, t73742: F, t73744: F, t73750: F, t85514: F, t85516: F, t85532: F) -> (F, F) {
    let t85543 = t9816 * t13847 * t73856 * t1883;
    let t85545 = t9962 * t22895;
    let t85548 = t125 * t22813;
    let t85553 = t125 * t22857;
    let t85562 = -F::cast_from(0.7623000421392799234e-3_f64) * t85514 - F::cast_from(0.12004725073059526352e-1_f64) * t85516 - F::cast_from(0.51448821741683684367e-2_f64) * t5671 * t3936 * t22046 * t4003 * t5591 + F::cast_from(0.25724410870841842184e-1_f64) * t5671 * t13783 * t6862 * t5627 + F::cast_from(0.48018900292238105408e-1_f64) * t73726 + F::cast_from(0.34299214494455789578e-3_f64) * t73729 + F::cast_from(0.22869001264178397701e-3_f64) * t73734 - F::cast_from(0.30492001685571196936e-3_f64) * t85532 + F::cast_from(0.15246000842785598467e-3_f64) * t73738 - F::cast_from(0.76230004213927992336e-4_f64) * t73742 + F::cast_from(0.18007087609589289529e-1_f64) * t73744 - F::cast_from(0.64311027177104605458e-3_f64) * t3934 * t5673 * t22079 * t5659 - F::cast_from(0.38115002106963996169e-4_f64) * t85543 + F::cast_from(0.60023625365297631763e-1_f64) * t85545 + F::cast_from(0.60023625365297631762e-1_f64) * t73750 + F::cast_from(0.25724410870841842183e-1_f64) * t3934 * t47248 * t85548 * t1399 + F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t3936 * t85553 * t3938 - F::cast_from(0.64311027177104605458e-3_f64) * t3934 * t5673 * t22046 * t5659;
    (t85553, t85562)
}
