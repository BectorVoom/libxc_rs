//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3304/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3304<F: Float>(t22974: F, t47603: F, t686: F, t72: F, t213: F, t22964: F, t13729: F, t2782: F, t556: F, t6918: F, t1445: F, t22390: F, t22414: F, t22975: F, t4071: F, t47601: F, t47618: F, t47793: F, t47794: F, t49513: F, t5775: F, t74829: F, t74836: F, t74838: F, t74843: F, t74849: F, t74853: F, t75336: F) -> F {
    let t86699 = t47603 * t22974 * t72 * t686;
    let t86701 = t213 * t22964;
    let t86712 = t2782 * t556 * t13729 * t6918;
    let t86718 = -F::cast_from(0.39512695097613069591e1_f64) * t4071 * t22975 + F::cast_from(0.32927245914677557992e-1_f64) * t74829 + t47601 + t49513 - F::cast_from(0.29272321618148349057e-1_f64) * t74836 + F::cast_from(0.7805952431506226415e-1_f64) * t74838 - F::cast_from(0.58544643236296698112e-1_f64) * t86699 - F::cast_from(0.65854491829355115987e0_f64) * t86701 * t1445 - F::cast_from(0.17563392970889009434e0_f64) * t74843 - F::cast_from(0.43902994552903410656e-1_f64) * t74849 - F::cast_from(0.11853808529283920877e2_f64) * t47793 * t47794 * t22414 - F::cast_from(0.16463622957338778996e-1_f64) * t74853 - F::cast_from(0.32927245914677557992e-1_f64) * t86712 - F::cast_from(0.19756347548806534796e1_f64) * t22390 * t5775 + F::cast_from(0.11708928647259339623e0_f64) * t75336 + F::cast_from(0.26019841438354088051e-2_f64) * t47618;
    t86718
}
