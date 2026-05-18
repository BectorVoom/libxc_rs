//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 920/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk920<F: Float>(t16043: F, t8504: F, t2186: F, t8582: F, t2347: F, t833: F, t262: F, t8640: F, t848: F, t7198: F, t2024: F, t27059: F, t35473: F, t39827: F, t39830: F, t39833: F, t39838: F, t39841: F, t39842: F, t39855: F, t39859: F, t39861: F, t39864: F, t39869: F, t4044: F, t5181: F, t665: F, t739: F) -> (F, F, F, F, F) {
    let t39871 = t16043 * t8504;
    let t39873 = t2186 * t8582;
    let t39874 = F::new(0.19863479950205658386e-4) * t39873;
    let t39875 = t2347 * t833;
    let t39876 = t262 * t39875;
    let t39877 = t8640 * t39876;
    let t39879 = t2347 * t848;
    let t39880 = t262 * t39879;
    let t39881 = t7198 * t39880;
    let t39884 = -t39827 - F::new(0.42564599893297839398e-5) * t39830 - F::new(0.85129199786595678796e-5) * t39833 - F::new(0.11971293719990017331e-4) * t39838 - t39841 + F::new(0.59590439850616975156e-4) * t39842 - F::new(0.71845450211182851384e0) * t4044 * t665 * t5181 + F::new(0.11974241701863808564e0) * t739 * t2024 * t27059 - F::new(0.17025839957319135759e-4) * t39855 + F::new(0.17025839957319135759e-4) * t39859 + F::new(0.25538759935978703638e-4) * t39861 + F::new(0.34093327067806677161e-2) * t39864 - F::new(0.1064114997332445985e-4) * t39869 + F::new(0.25538759935978703638e-4) * t39871 + t39874 + F::new(0.20455996240684006296e-1) * t39877 - F::new(0.40911992481368012592e-1) * t39881 + F::new(0.99317399751028291929e-5) * t35473;
    (t39875, t39876, t39879, t39880, t39884)
}
