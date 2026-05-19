//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1373/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1373<F: Float>(t10792: F, t10795: F, t10799: F, t10802: F, t10806: F, t10808: F, t10811: F, t10813: F, t10817: F, t10823: F, t10825: F, t10828: F, t10831: F, t10834: F, t10838: F, t117: F, t123: F, t125: F, t14500: F, t15210: F, t15253: F, t15288: F, t15464: F, t15494: F, t15700: F, t15732: F, t15766: F, t15800: F, t15833: F, t15876: F, t15916: F, t15960: F, t15997: F, t16037: F, t16078: F, t16110: F, t16142: F, t16186: F, t16221: F, t16253: F, t16290: F, t16316: F, t16368: F, t16394: F, t16422: F, t16454: F, t16500: F, t16539: F, t16575: F, t16607: F, t16641: F, t16686: F, t16720: F, t16753: F, t16791: F, t16838: F, t16871: F, t16914: F, t16946: F, t16977: F, t17010: F, t17260: F, t17291: F, t17381: F, t17425: F, t17479: F, t17518: F, t17562: F, t17591: F, t17641: F, t17677: F, t17705: F, t17729: F, t17765: F, t17779: F, t17816: F, t17840: F, t17857: F, t17881: F, t17916: F, t17947: F, t17986: F, t18025: F) -> F {
    let t18044 = -t10792 - F::cast_from(0.09579387208203688_f64) * t14500 - F::cast_from(0.005388405304614574_f64) * t123 * t125 * (t17947 + t17916 + t17881 + t17857 + t17840 + t17816 + t17779 + t17765 + t17729 + t17705 + t17677 + t17641 + t17591 + t17562 + t17518 + t17479 + t17425 + t17381 + t17291 + t17260 + t17010 + t16977 + t16946 + t16914 + t16871 + t16838 + t16791 + t16753 + t16720 + t16686 + t16641 + t16607 + t16575 + t16539 + t16500 + t16454 + t16422 + t16394 + t16368 + t16316 + t16290 + t16253 + t16221 + t16186 + t16142 + t16037 + t15997 + t15960 + t15916 + t15876 + t15833 + t15800 + t15766 + t15732 + t15700 + t15494 + t15464 + t16110 + t15288 + t15253 + t16078 + t15210 + t17986 + t18025) * t117 + F::cast_from(0.006935985972286697_f64) * t10795 - F::cast_from(0.0004954275694490498_f64) * t10799 - F::cast_from(0.002972565416694299_f64) * t10802 - t10806 - t10808 - t10811 - F::cast_from(0.01185233419734569_f64) * t10813 + t10817 - F::cast_from(0.003950778065781896_f64) * t10823 - F::cast_from(0.051799090195807085_f64) * t10825 - t10828 + F::cast_from(0.006584630109636494_f64) * t10831 + F::cast_from(0.03950778065781896_f64) * t10834 + t10838;
    t18044
}
