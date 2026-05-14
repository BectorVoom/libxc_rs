//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1436/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1436<F: Float>(t26274: F, t6086: F, t6093: F, t6082: F, t980: F, t20470: F, t7418: F, t2294: F, t2598: F, t7434: F, t7438: F, t11: F, t146: F, t147: F, t1593: F, t20040: F, t21036: F, t21038: F, t21041: F, t21044: F, t26340: F, t26356: F, t26360: F, t26362: F, t26372: F, t26392: F, t26408: F, t26415: F, t26434: F, t26440: F, t26457: F, t26468: F, t26477: F, t26495: F, t26519: F, t26539: F, t26552: F, t26577: F, t26593: F, t26617: F, t26637: F, t26645: F, t26652: F, t26677: F, t26698: F, t26726: F, t26746: F, t26760: F, t26768: F, t26792: F, t26812: F, t26826: F, t26834: F, t26847: F, t26852: F, t26863: F, t26877: F, t26883: F, t26890: F, t26895: F, t26911: F, t26919: F, t26929: F, t26934: F, t26946: F, t26950: F, t26962: F, t26972: F, t26975: F, t26980: F, t26983: F, t26984: F, t26999: F, t279: F, t360: F, t5: F, t6149: F, t6152: F, t6254: F, t7443: F, t7461: F, t7946: F, t7977: F, t7984: F, t8112: F) -> (F,) {
    let t27002 = t6093 * t6086 * t26274;
    let t27004 = t980 * t6082;
    let t27006 = t20470 * t7418;
    let t27009 = t2598 * t2294 * t7434;
    let t27012 = t2598 * t2294 * t7438;
    let t27014 = 0.39006997830244208535e0 * t6152 * t7946 - 0.34672886960217074253e0 * t26340 - 0.26004665220162805689e0 * t20040 * t8112 - 0.31205598264195366828e1 * t7461 * t360 * t7977 * t1593 + 0.13002332610081402845e0 * t7984 * t6254 + 0.26004665220162805689e0 * t6149 * t7443 + 0.43341108700271342816e-1 * t146 * t147 * (t21036 - 140.0 / 9.0 * t21038 + 20.0 / 3.0 * t21041 - 5.0 / 3.0 * t21044 - 140.0 / 27.0 * t26356 + t26360 - 5.0 * t26362 + 5.0 * t5 * t11 * (t26847 + t26826 + t26617 + t26792 + t26863 + t26539 + t26645 + t26768 + t26392 + t26477 + t26812 + t26593 + t26637 + t26408 + t26415 + t26495 + t26652 + t26677 + t26457 + t26372 + t26852 + t26760 + t26577 + t26746 + t26519 + t26552 + t26834 + t26434 + t26726 + t26468 + t26440 + t26698) - 45.0 * param_eta * (t26877 + t26883 + t26890 + t26895 + t26911 + t26919 + t26929 + t26934 + t26946 + t26950 + t26962 + t26972 + t26975 + t26980 + t26983 + t26984)) * t279 + 0.17465477326173296717e-1 * t26999 + 0.1047928639570397803e0 * t27002 - 0.1713958891116262235e0 * t27004 - 0.17888640988868435534e-2 * t27006 - 0.13869154784086829701e1 * t27009 - 0.69345773920434148506e0 * t27012;
    (t27014,)
}
