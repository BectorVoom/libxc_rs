//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1256/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1256<F: Float>(t107: F, t110: F, t11726: F, t11731: F, t11733: F, t11745: F, t11747: F, t122: F, t14232: F, t14234: F, t18141: F, t18144: F, t19205: F, t19596: F, t20068: F, t202: F, t21255: F, t21827: F, t21850: F, t21851: F, t21855: F, t21897: F, t21901: F, t21904: F, t21908: F, t21909: F, t21911: F, t21912: F, t21915: F, t21916: F, t21922: F, t21925: F, t21927: F, t21930: F, t21935: F, t21938: F, t21940: F, t21944: F, t21948: F, t21949: F, t21951: F, t21955: F, t21956: F, t21970: F, t21972: F, t21978: F, t21979: F, t21981: F, t21982: F, t21987: F, t21988: F, t21990: F, t21991: F, t21995: F, t21997: F, t22000: F, t22001: F, t22005: F, t22006: F, t22008: F, t22009: F, t22014: F, t22015: F, t22017: F, t22021: F, t22024: F, t22025: F, t22027: F, t22028: F, t22036: F, t22037: F, t22039: F, t22040: F, t22043: F, t22044: F) -> F {
    let t22063 = -F::new(0.011938374665504766) * t122 * t202 * (t21904 + t21909 + t21908 + t21901 + t21935 + t20068 + t22008 + t22009 + t21897 + t21930 + t21938 + t22014 + t21944 + t21940 + t22021 + t21255 + t22037 + t22039 + t22040 + t22005 + t22006 + t22015 + t22017 + t21925 + t21927 + t21987 + t22027 + t22028 + t21995 + t22024 + t22025 + t21855 + t21850 + t21851 + t21915 + t21916 + t21922 + t21988 + t21990 + t21991 + t22000 + t22001 + t21911 + t21912 + t22043 + t22044 + t21981 + t21982 + t22036 + t21997 + t21970 + t21972 + t19205 + t21978 + t21979 + t21948 + t21949 + t21951 + t21955 + t21956 + t19596) + F::new(0.42708890021612717) * t107 * t110 * t21827 + F::new(0.5836538725357885) * t11726 + F::new(1.5077307696390791) * t11731 + F::new(1.5077307696390791) * t11733 + t11745 - F::new(13.28721022894618) * t11747 - F::new(1.7083556008645087) * t18141 - F::new(0.15917832887339686) * t18144 - t14232 - t14234;
    t22063
}
