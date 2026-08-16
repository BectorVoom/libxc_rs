//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1540/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1540(t11671: f64, t11926: f64, t1045: f64, t2862: f64, t999: f64, t3075: f64, t606: f64, t1042: f64, t1063: f64, t11678: f64, t11774: f64, t11930: f64, t15785: f64, t15906: f64, t16043: f64, t16081: f64, t16089: f64, t16101: f64, t16199: f64, t3092: f64, t3115: f64, t3117: f64, t42309: f64, t42804: f64, t43180: f64, t43277: f64, t43279: f64, t43285: f64, t43288: f64, t43291: f64, t43292: f64) -> f64 {
    let t43297 = t11926 * t11671;
    let t43301 = t1045 * t2862 * t999;
    let t43313 = t606 * t3075;
    let t43318 = -0.85748036236139473944e-2_f64 * t1063 * t1042 * t16199 * t43180 - 0.17149607247227894789e-2_f64 * t43277 + 0.77173232612525526552e-2_f64 * t16081 * t3117 * t42804 * t43279 + 0.51448821741683684368e-2_f64 * t43285 * t11930 - 0.34299214494455789577e-2_f64 * t43288 - 0.51448821741683684368e-2_f64 * t43291 * t3117 * t43292 * t1045 - 0.27439371595564631662e-1_f64 * t43297 * t11930 - 0.17149607247227894789e-2_f64 * t11774 * t42309 * t43301 - 0.77173232612525526552e-2_f64 * t15906 * t3117 * t42804 * t15785 - 0.12862205435420921092e-2_f64 * t3115 * t3117 * t11678 * t16043 + 0.34299214494455789578e-2_f64 * t16089 * t3092 * t16101 * t43313;
    t43318
}
