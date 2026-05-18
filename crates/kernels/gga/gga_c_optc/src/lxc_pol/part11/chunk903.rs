//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 903/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk903<F: Float>(t17004: F, t894: F, t16231: F, t897: F, t16988: F, t297: F, t935: F, t313: F, t16225: F, t7870: F, t14253: F, t14280: F, t14285: F, t14290: F, t14308: F, t16962: F, t16965: F, t16969: F, t16976: F, t16981: F, t16985: F, t16991: F, t16994: F, t16998: F, t17001: F, t2640: F, t2668: F, t2678: F, t7372: F, t7386: F, t862: F, t893: F) -> (F, F, F, F, F, F, F) {
    let t17005 = t894 * t17004;
    let t17008 = t897 * t16231;
    let t17009 = t894 * t17008;
    let t17013 = t16988 * t935 * t297;
    let t17014 = t313 * t17013;
    let t17017 = t7870 * t16225;
    let t17018 = t894 * t17017;
    let t17022 = F::new(0.94667510637550784468e-1) * t14253 + F::new(0.27471835441379490507e2) * t2668 * t16962 + F::new(0.71000632978163088351e-1) * t2640 * t16965 + F::new(0.71000632978163088351e-1) * t2640 * t16969 - F::new(0.72443012414906315455e-2) * t14280 + F::new(0.36221506207453157727e-2) * t14285 + F::new(0.60369177012421929545e-2) * t14290 + F::new(0.11833438829693848058e0) * t2640 * t16976 - F::new(0.13735917720689745254e2) * t2678 * t16981 - F::new(0.1420012659563261767e0) * t2640 * t16985 - F::new(0.10629507243271336419e5) * t7386 * t16991 - t862 * t16994 / F::new(48.0) + F::new(0.10866451862235947318e-1) * t893 * t16998 + t862 * t17001 / F::new(72.0) + F::new(0.80492236016562572728e-2) * t893 * t17005 + F::new(0.18110753103726578864e-2) * t893 * t17009 + F::new(0.17715845405452227366e4) * t7372 * t17014 - F::new(0.18110753103726578864e-1) * t893 * t17018 - F::new(0.91572784804598301689e1) * t14308;
    (t17005, t17008, t17009, t17013, t17017, t17018, t17022)
}
