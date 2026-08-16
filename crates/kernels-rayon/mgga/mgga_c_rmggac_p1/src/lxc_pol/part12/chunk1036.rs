//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1036/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1036(t2412: f64, t7914: f64, t3351: f64, t3352: f64, t5181: f64, t880: f64, t1243: f64, t515: f64, t570: f64, t7231: f64, t27059: f64, t1356: f64, t36475: f64, t36499: f64, t40772: f64, t40776: f64, t40780: f64, t40785: f64, t40816: f64, t40848: f64, t40876: f64, t40915: f64, t40953: f64, t40988: f64, t41026: f64, t41069: f64, t41097: f64, t41126: f64, t41420: f64, t41452: f64, t41482: f64, t41484: f64, t41511: f64, t41533: f64, t41564: f64, t41571: f64, t41577: f64, t41579: f64, t41582: f64, t41585: f64, t72: f64, t739: f64, t82: f64) -> f64 {
    let t41587 = t2412 * t7914;
    let t41591 = t3351 * t3352 * t880 * t5181;
    let t41596 = t3351 * t7231 * t515 * t570 * t1243;
    let t41600 = t3351 * t3352 * t515 * t27059;
    let t41602 = 0.1064114997332445985e-4_f64 * t40772 + 0.25538759935978703638e-4_f64 * t40776 - 0.25538759935978703638e-4_f64 * t40780 + 2.0_f64 * t36475 + t72 * t82 * (t40816 + t40848 + t40876 + t40915 + t40953 + t40988 + t41026 + t41069 + t41097 + t41126 + t41420 + t41452 + t41482 + t41511 + t41533 + t41564) + t41571 - 0.59871208509319042821e-1_f64 * t739 * t40785 + 0.85129199786595678796e-5_f64 * t41577 + 0.74488049813271218945e-4_f64 * t41579 + t41582 + t36499 + 0.79828278012425390428e-1_f64 * t1356 * t41484 - 0.59590439850616975156e-4_f64 * t41585 + 0.51077519871957407276e-4_f64 * t41587 + 0.15323255961587222183e-3_f64 * t41591 + 0.42564599893297839398e-5_f64 * t41596 - 0.12769379967989351819e-4_f64 * t41600;
    t41602
}
