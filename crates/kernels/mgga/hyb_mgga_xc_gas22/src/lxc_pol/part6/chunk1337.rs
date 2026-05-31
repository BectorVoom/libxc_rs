//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1337/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1337<F: Float>(t4193: F, t6669: F, t10781: F, t10809: F, t10817: F, t10820: F, t10826: F, t10830: F, t20744: F, t21037: F, t21053: F, t2252: F, t2291: F, t2306: F, t2312: F, t24813: F, t24916: F, t3371: F, t3390: F, t3422: F, t4154: F, t4180: F, t4181: F, t4194: F, t6641: F, t6667: F, t6678: F, t6683: F, t6722: F, t8709: F, t8785: F, t8795: F, t8815: F, t8818: F, t8821: F, t8862: F, t8916: F) -> F {
    let t29098 = t4193 * t6669;
    let t29128 = -F::cast_from(0.14035736694323150897e2_f64) * t6641 * t4181 * t2291 + F::cast_from(0.70178683471615754484e1_f64) * t8916 * t8815 - F::cast_from(0.24828486201251232145e5_f64) * t21053 * t10781 * t2252 + F::cast_from(0.35089341735807877242e1_f64) * t2312 * t4194 * t2291 - F::cast_from(24.0_f64) * t6683 * t4154 * t2252 + F::cast_from(12.0_f64) * t8862 * t8818 + F::cast_from(0.10254018858216406658e4_f64) * t6667 * t29098 * t2291 + F::cast_from(0.34631718211362927518e2_f64) * t2312 * t3422 * t8709 + F::cast_from(0.10254018858216406658e4_f64) * t6667 * t10809 * t2306 + F::cast_from(0.91082604192152556044e5_f64) * t21037 * t4180 * t20744 * t2291 - F::cast_from(8.0_f64) * t24813 * t3371 + F::cast_from(0.12865583598954028054e3_f64) * t24916 * t3390 - F::cast_from(8.0_f64) * t8821 * t8785 + F::cast_from(0.12865583598954028054e3_f64) * t8862 * t8795 + F::cast_from(12.0_f64) * t6678 * t10817 - F::cast_from(8.0_f64) * t6722 * t10820 - F::cast_from(4.0_f64) * t6722 * t10826 + F::cast_from(0.64327917994770140268e2_f64) * t6678 * t10830;
    t29128
}
