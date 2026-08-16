//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1337/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1337(t4193: f64, t6669: f64, t10781: f64, t10809: f64, t10817: f64, t10820: f64, t10826: f64, t10830: f64, t20744: f64, t21037: f64, t21053: f64, t2252: f64, t2291: f64, t2306: f64, t2312: f64, t24813: f64, t24916: f64, t3371: f64, t3390: f64, t3422: f64, t4154: f64, t4180: f64, t4181: f64, t4194: f64, t6641: f64, t6667: f64, t6678: f64, t6683: f64, t6722: f64, t8709: f64, t8785: f64, t8795: f64, t8815: f64, t8818: f64, t8821: f64, t8862: f64, t8916: f64) -> f64 {
    let t29098 = t4193 * t6669;
    let t29128 = -0.14035736694323150897e2_f64 * t6641 * t4181 * t2291 + 0.70178683471615754484e1_f64 * t8916 * t8815 - 0.24828486201251232145e5_f64 * t21053 * t10781 * t2252 + 0.35089341735807877242e1_f64 * t2312 * t4194 * t2291 - 24.0_f64 * t6683 * t4154 * t2252 + 12.0_f64 * t8862 * t8818 + 0.10254018858216406658e4_f64 * t6667 * t29098 * t2291 + 0.34631718211362927518e2_f64 * t2312 * t3422 * t8709 + 0.10254018858216406658e4_f64 * t6667 * t10809 * t2306 + 0.91082604192152556044e5_f64 * t21037 * t4180 * t20744 * t2291 - 8.0_f64 * t24813 * t3371 + 0.12865583598954028054e3_f64 * t24916 * t3390 - 8.0_f64 * t8821 * t8785 + 0.12865583598954028054e3_f64 * t8862 * t8795 + 12.0_f64 * t6678 * t10817 - 8.0_f64 * t6722 * t10820 - 4.0_f64 * t6722 * t10826 + 0.64327917994770140268e2_f64 * t6678 * t10830;
    t29128
}
