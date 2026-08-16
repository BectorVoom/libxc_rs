//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1286/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1286(t41855: f64, t41878: f64, t41931: f64, t41975: f64, t2853: f64, t2860: f64, t10770: f64, t919: f64, t2862: f64, t10655: f64, t10737: f64, t10632: f64, t10753: f64, t10757: f64, t10772: f64, t10805: f64, t10806: f64, t10811: f64, t10813: f64, t10820: f64, t2861: f64, t2863: f64, t2880: f64, t2886: f64, t2888: f64, t2900: f64, t2907: f64, t2924: f64, t2925: f64, t2930: f64, t2933: f64, t41804: f64, t41813: f64, t41816: f64, t41821: f64, t41826: f64, t41827: f64, t931: f64, t943: f64, t951: f64) -> (f64, f64, f64, f64) {
    let t41977 = t41855 + t41878 + t41931 + t41975;
    let t41981 = t2853 * t2860;
    let t41984 = t919 * t10770;
    let t41987 = t2862 * t2862;
    let t41992 = 24.0_f64 * t10655 * t10737;
    let t41993 = -8.0_f64 * t2861 * t10806 * t931 + 0.12865583598954028054e3_f64 * t2886 * t10805 * t2888 * t931 + 0.12414243100625616072e5_f64 * t10811 * t2862 * t10813 * t2880 - t41804 + 36.0_f64 * t2886 * t2863 * t2880 + 0.21053605041484726346e2_f64 * t2930 * t2907 * t2924 + t41813 + 0.35089341735807877242e1_f64 * t10820 * t2925 + 0.10389515463408878255e3_f64 * t41816 * t2933 + 0.23392894490538584828e1_f64 * t2900 * t10753 + 0.4101607543286562663e4_f64 * t41821 * t10757 - 0.12304822629859687989e5_f64 * t41826 * t41827 * t10632 + 0.5848223622634646207e0_f64 * t943 * t41977 * t951 - 12.0_f64 * t41981 * t2863 - 0.77193501593724168322e3_f64 * t41984 * t10772 + 0.11579025239058625248e4_f64 * t10811 * t41987 * t2888 - t41992;
    (t41977, t41987, t41992, t41993)
}
