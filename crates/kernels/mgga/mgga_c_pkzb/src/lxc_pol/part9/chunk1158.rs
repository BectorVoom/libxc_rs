//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1158/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1158<F: Float>(t20201: F, t17043: F, t6888: F, t17053: F, t2602: F, t1020: F, t1634: F, t2587: F, t5264: F, t1024: F, t16343: F, t17026: F, t17034: F, t1706: F, t17067: F, t1719: F, t1733: F, t1734: F, t179: F, t20164: F, t20166: F, t20168: F, t20195: F, t2593: F, t5217: F, t5244: F, t5279: F, t568: F, t581: F, t600: F, t6859: F, t6875: F, t6880: F, t6896: F, t6897: F, t6939: F, t6956: F, t6979: F) -> F {
    let t20202 = F::cast_from(0.34013387707001991332e-1_f64) * t20201;
    let t20203 = t17043 * t6888;
    let t20205 = t17053 * t2602;
    let t20212 = t1020 * t1634;
    let t20221 = t5264 * t2587;
    let t20222 = F::new(35.0) / F::new(72.0) * t20221;
    let t20223 = -F::cast_from(0.68026775414003982662e-1_f64) * t20164 - F::cast_from(0.24009450146119052704e-1_f64) * t20166 + F::cast_from(0.25724410870841842184e-1_f64) * t16343 * t179 * t6875 * t20168 + F::cast_from(0.25724410870841842183e-2_f64) * t1733 * t179 * t6859 * t6939 + t17026 + F::cast_from(0.51448821741683684367e-2_f64) * t1733 * t179 * t6979 * t6939 - F::cast_from(0.1543464652250510531e-1_f64) * t5244 * t179 * t6880 * t20168 + F::cast_from(0.1543464652250510531e-1_f64) * t17067 * t179 * t6897 * t1719 * t568 - F::cast_from(0.25724410870841842183e-1_f64) * t5279 * t179 * t6956 * t6939 - F::cast_from(0.38586616306262763275e-2_f64) * t6896 * t179 * t20195 + t20202 + F::cast_from(0.60023625365297631762e-1_f64) * t20203 + F::cast_from(0.68026775414003982663e-1_f64) * t20205 + F::cast_from(0.25724410870841842183e-1_f64) * t16343 * t179 * t2593 * t1634 * t600 + F::cast_from(0.77173232612525526549e-1_f64) * t17034 * t179 * t20212 * t1734 + t1706 * t581 * t1024 * t5217 / F::new(16.0) - t20222;
    t20223
}
