//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1030/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1030<F: Float>(t14245: F, t11745: F, t11747: F, t11773: F, t11787: F, t11812: F, t11838: F, t11854: F, t11887: F, t11913: F, t11947: F, t11963: F, t11995: F, t12033: F, t12072: F, t12104: F, t12126: F, t12171: F, t122: F, t12211: F, t12226: F, t12257: F, t12303: F, t12442: F, t12470: F, t12512: F, t12572: F, t12613: F, t12646: F, t12667: F, t12714: F, t12754: F, t12789: F, t12819: F, t12837: F, t12872: F, t12901: F, t12971: F, t12999: F, t13050: F, t13083: F, t13109: F, t13141: F, t13173: F, t13203: F, t13228: F, t13254: F, t13273: F, t13293: F, t13325: F, t13441: F, t13471: F, t13506: F, t13669: F, t13704: F, t13735: F, t13760: F, t13796: F, t13821: F, t13830: F, t13857: F, t13889: F, t13919: F, t13952: F, t13993: F, t14021: F, t14222: F, t14232: F, t14234: F, t14236: F, t14238: F, t14240: F, t14242: F, t202: F, t9063: F) -> (F,) {
    let t14246 = 0.15917832887339686 * t14245;
    let t14247 = -1.0051538464260528 * t9063 + t11745 - 4.429070076315393 * t11747 - 0.011938374665504766 * t122 * t202 * (t14222 + t13993 + t13952 + t13919 + t13889 + t14021 + t13857 + t13830 + t13821 + t13796 + t13760 + t13735 + t13704 + t13669 + t13506 + t13471 + t13441 + t13325 + t13293 + t13273 + t13254 + t13228 + t13203 + t13173 + t13141 + t13109 + t13083 + t13050 + t12999 + t12971 + t12901 + t12872 + t12837 + t12442 + t12789 + t12470 + t11995 + t12072 + t11854 + t12512 + t12667 + t11838 + t11947 + t12211 + t12572 + t12819 + t12126 + t12033 + t12714 + t11787 + t12257 + t11887 + t11963 + t12613 + t12303 + t12646 + t11773 + t11913 + t12226 + t11812 + t12104 + t12171 + t12754) - t14232 - t14234 - t14236 - t14238 + 0.2512884616065132 * t14240 + 0.5025769232130264 * t14242 - t14246;
    (t14247,)
}
