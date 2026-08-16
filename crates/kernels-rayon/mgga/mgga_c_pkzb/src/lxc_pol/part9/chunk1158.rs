//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1158/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1158(t20201: f64, t17043: f64, t6888: f64, t17053: f64, t2602: f64, t1020: f64, t1634: f64, t2587: f64, t5264: f64, t1024: f64, t16343: f64, t17026: f64, t17034: f64, t1706: f64, t17067: f64, t1719: f64, t1733: f64, t1734: f64, t179: f64, t20164: f64, t20166: f64, t20168: f64, t20195: f64, t2593: f64, t5217: f64, t5244: f64, t5279: f64, t568: f64, t581: f64, t600: f64, t6859: f64, t6875: f64, t6880: f64, t6896: f64, t6897: f64, t6939: f64, t6956: f64, t6979: f64) -> f64 {
    let t20202 = 0.34013387707001991332e-1_f64 * t20201;
    let t20203 = t17043 * t6888;
    let t20205 = t17053 * t2602;
    let t20212 = t1020 * t1634;
    let t20221 = t5264 * t2587;
    let t20222 = 35.0_f64 / 72.0_f64 * t20221;
    let t20223 = -0.68026775414003982662e-1_f64 * t20164 - 0.24009450146119052704e-1_f64 * t20166 + 0.25724410870841842184e-1_f64 * t16343 * t179 * t6875 * t20168 + 0.25724410870841842183e-2_f64 * t1733 * t179 * t6859 * t6939 + t17026 + 0.51448821741683684367e-2_f64 * t1733 * t179 * t6979 * t6939 - 0.1543464652250510531e-1_f64 * t5244 * t179 * t6880 * t20168 + 0.1543464652250510531e-1_f64 * t17067 * t179 * t6897 * t1719 * t568 - 0.25724410870841842183e-1_f64 * t5279 * t179 * t6956 * t6939 - 0.38586616306262763275e-2_f64 * t6896 * t179 * t20195 + t20202 + 0.60023625365297631762e-1_f64 * t20203 + 0.68026775414003982663e-1_f64 * t20205 + 0.25724410870841842183e-1_f64 * t16343 * t179 * t2593 * t1634 * t600 + 0.77173232612525526549e-1_f64 * t17034 * t179 * t20212 * t1734 + t1706 * t581 * t1024 * t5217 / 16.0_f64 - t20222;
    t20223
}
