//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1375/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1375<F: Float>(t44951: F, t5330: F, t3362: F, t404: F, t43766: F, t13026: F, t43776: F, t43813: F, t3450: F, t3475: F, t426: F, t43816: F) -> (F, F, F, F, F, F, F) {
    let t44952 = t44951 * t5330;
    let t44958 = F::cast_from(1.0_f64) / t404 / t3362;
    let t44959 = t44958 * t43766;
    let t44974 = t13026 * t43776;
    let t45000 = F::cast_from(0.18467901234567901234e0_f64) * t43813;
    let t45085 = t426 / t3475 / t3450;
    let t45106 = F::cast_from(0.5356037037037037037e1_f64) * t43813;
    let t45107 = F::cast_from(0.16979925925925925926e1_f64) * t43816;
    (t44952, t44959, t44974, t45000, t45085, t45106, t45107)
}
