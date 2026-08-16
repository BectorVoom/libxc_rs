//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3304/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3304(t22974: f64, t47603: f64, t686: f64, t72: f64, t213: f64, t22964: f64, t13729: f64, t2782: f64, t556: f64, t6918: f64, t1445: f64, t22390: f64, t22414: f64, t22975: f64, t4071: f64, t47601: f64, t47618: f64, t47793: f64, t47794: f64, t49513: f64, t5775: f64, t74829: f64, t74836: f64, t74838: f64, t74843: f64, t74849: f64, t74853: f64, t75336: f64) -> f64 {
    let t86699 = t47603 * t22974 * t72 * t686;
    let t86701 = t213 * t22964;
    let t86712 = t2782 * t556 * t13729 * t6918;
    let t86718 = -0.39512695097613069591e1_f64 * t4071 * t22975 + 0.32927245914677557992e-1_f64 * t74829 + t47601 + t49513 - 0.29272321618148349057e-1_f64 * t74836 + 0.7805952431506226415e-1_f64 * t74838 - 0.58544643236296698112e-1_f64 * t86699 - 0.65854491829355115987e0_f64 * t86701 * t1445 - 0.17563392970889009434e0_f64 * t74843 - 0.43902994552903410656e-1_f64 * t74849 - 0.11853808529283920877e2_f64 * t47793 * t47794 * t22414 - 0.16463622957338778996e-1_f64 * t74853 - 0.32927245914677557992e-1_f64 * t86712 - 0.19756347548806534796e1_f64 * t22390 * t5775 + 0.11708928647259339623e0_f64 * t75336 + 0.26019841438354088051e-2_f64 * t47618;
    t86718
}
