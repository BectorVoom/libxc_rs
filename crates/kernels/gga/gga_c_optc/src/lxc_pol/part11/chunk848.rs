//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 848/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk848<F: Float>(t17017: F, t894: F, t14253: F, t14280: F, t14285: F, t14290: F, t14308: F, t16962: F, t16965: F, t16969: F, t16976: F, t16981: F, t16985: F, t16991: F, t16994: F, t16998: F, t17001: F, t17005: F, t17009: F, t17014: F, t2640: F, t2668: F, t2678: F, t7372: F, t7386: F, t862: F, t893: F) -> (F, F) {
    let t17018 = t894 * t17017;
    let t17022 = 0.94667510637550784468e-1 * t14253 + 0.27471835441379490507e2 * t2668 * t16962 + 0.71000632978163088351e-1 * t2640 * t16965 + 0.71000632978163088351e-1 * t2640 * t16969 - 0.72443012414906315455e-2 * t14280 + 0.36221506207453157727e-2 * t14285 + 0.60369177012421929545e-2 * t14290 + 0.11833438829693848058e0 * t2640 * t16976 - 0.13735917720689745254e2 * t2678 * t16981 - 0.1420012659563261767e0 * t2640 * t16985 - 0.10629507243271336419e5 * t7386 * t16991 - t862 * t16994 / 48.0 + 0.10866451862235947318e-1 * t893 * t16998 + t862 * t17001 / 72.0 + 0.80492236016562572728e-2 * t893 * t17005 + 0.18110753103726578864e-2 * t893 * t17009 + 0.17715845405452227366e4 * t7372 * t17014 - 0.18110753103726578864e-1 * t893 * t17018 - 0.91572784804598301689e1 * t14308;
    (t17018, t17022)
}
