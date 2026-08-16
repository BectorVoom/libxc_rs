//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 920/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk920(t16043: f64, t8504: f64, t2186: f64, t8582: f64, t2347: f64, t833: f64, t262: f64, t8640: f64, t848: f64, t7198: f64, t2024: f64, t27059: f64, t35473: f64, t39827: f64, t39830: f64, t39833: f64, t39838: f64, t39841: f64, t39842: f64, t39855: f64, t39859: f64, t39861: f64, t39864: f64, t39869: f64, t4044: f64, t5181: f64, t665: f64, t739: f64) -> (f64, f64, f64, f64, f64) {
    let t39871 = t16043 * t8504;
    let t39873 = t2186 * t8582;
    let t39874 = 0.19863479950205658386e-4_f64 * t39873;
    let t39875 = t2347 * t833;
    let t39876 = t262 * t39875;
    let t39877 = t8640 * t39876;
    let t39879 = t2347 * t848;
    let t39880 = t262 * t39879;
    let t39881 = t7198 * t39880;
    let t39884 = -t39827 - 0.42564599893297839398e-5_f64 * t39830 - 0.85129199786595678796e-5_f64 * t39833 - 0.11971293719990017331e-4_f64 * t39838 - t39841 + 0.59590439850616975156e-4_f64 * t39842 - 0.71845450211182851384e0_f64 * t4044 * t665 * t5181 + 0.11974241701863808564e0_f64 * t739 * t2024 * t27059 - 0.17025839957319135759e-4_f64 * t39855 + 0.17025839957319135759e-4_f64 * t39859 + 0.25538759935978703638e-4_f64 * t39861 + 0.34093327067806677161e-2_f64 * t39864 - 0.1064114997332445985e-4_f64 * t39869 + 0.25538759935978703638e-4_f64 * t39871 + t39874 + 0.20455996240684006296e-1_f64 * t39877 - 0.40911992481368012592e-1_f64 * t39881 + 0.99317399751028291929e-5_f64 * t35473;
    (t39875, t39876, t39879, t39880, t39884)
}
