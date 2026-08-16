//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3526/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3526<F: Float>(t4866: F, t4900: F, t1065: F, t372: F, t6299: F, t3105: F, t6317: F, t1068: F, t15689: F, t15692: F, t15907: F, t16067: F, t16089: F, t16128: F, t16226: F, t16229: F, t19705: F, t19819: F, t19831: F, t247: F, t3092: F, t3116: F, t3117: F, t43297: F, t4772: F, t4837: F, t54599: F, t54899: F, t606: F, t64912: F, t66752: F, t66758: F, t66763: F, t66766: F) -> (F, F, F) {
    let t66771 = t4900 * t4866;
    let t66777 = t372 * t1065 * t6299;
    let t66784 = t6317 * t3105;
    let t66793 = F::cast_from(0.11433071498151929859e-2_f64) * t66752 + F::cast_from(0.85748036236139473944e-3_f64) * t4837 * t247 * t3116 * t64912 - F::cast_from(0.57165357490759649296e-3_f64) * t66758 - F::cast_from(0.45732285992607719436e-2_f64) * t43297 * t19831 - F::cast_from(0.6351706387862183255e-4_f64) * t66763 - F::cast_from(0.95275595817932748826e-3_f64) * t66766 * t16128 - F::cast_from(0.25724410870841842184e-2_f64) * t54899 * t19819 + F::cast_from(0.85748036236139473944e-3_f64) * t16067 * t3117 * t15907 * t66771 + F::cast_from(0.57165357490759649296e-3_f64) * t16226 * t66777 * t16229 - F::cast_from(0.28582678745379824648e-3_f64) * t15689 * t66777 * t15692 - F::cast_from(0.15244095330869239812e-2_f64) * t66784 * t1068 + F::cast_from(0.1270341277572436651e-3_f64) * t54599 + F::cast_from(0.11433071498151929859e-2_f64) * t16089 * t3092 * t19705 * t606 * t4772;
    (t66771, t66777, t66793)
}
