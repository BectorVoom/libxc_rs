//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1036/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1036<F: Float>(t2412: F, t7914: F, t3351: F, t3352: F, t5181: F, t880: F, t1243: F, t515: F, t570: F, t7231: F, t27059: F, t1356: F, t36475: F, t36499: F, t40772: F, t40776: F, t40780: F, t40785: F, t40816: F, t40848: F, t40876: F, t40915: F, t40953: F, t40988: F, t41026: F, t41069: F, t41097: F, t41126: F, t41420: F, t41452: F, t41482: F, t41484: F, t41511: F, t41533: F, t41564: F, t41571: F, t41577: F, t41579: F, t41582: F, t41585: F, t72: F, t739: F, t82: F) -> F {
    let t41587 = t2412 * t7914;
    let t41591 = t3351 * t3352 * t880 * t5181;
    let t41596 = t3351 * t7231 * t515 * t570 * t1243;
    let t41600 = t3351 * t3352 * t515 * t27059;
    let t41602 = F::cast_from(0.1064114997332445985e-4_f64) * t40772 + F::cast_from(0.25538759935978703638e-4_f64) * t40776 - F::cast_from(0.25538759935978703638e-4_f64) * t40780 + F::new(2.0) * t36475 + t72 * t82 * (t40816 + t40848 + t40876 + t40915 + t40953 + t40988 + t41026 + t41069 + t41097 + t41126 + t41420 + t41452 + t41482 + t41511 + t41533 + t41564) + t41571 - F::cast_from(0.59871208509319042821e-1_f64) * t739 * t40785 + F::cast_from(0.85129199786595678796e-5_f64) * t41577 + F::cast_from(0.74488049813271218945e-4_f64) * t41579 + t41582 + t36499 + F::cast_from(0.79828278012425390428e-1_f64) * t1356 * t41484 - F::cast_from(0.59590439850616975156e-4_f64) * t41585 + F::cast_from(0.51077519871957407276e-4_f64) * t41587 + F::cast_from(0.15323255961587222183e-3_f64) * t41591 + F::cast_from(0.42564599893297839398e-5_f64) * t41596 - F::cast_from(0.12769379967989351819e-4_f64) * t41600;
    t41602
}
