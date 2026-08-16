//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 828/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk828(t7256: f64, t7856: f64, t6534: f64, t894: f64, t2722: f64, t7354: f64, t2263: f64, t896: f64, t2595: f64, t7298: f64, t2583: f64, t2591: f64, t2598: f64, t2640: f64, t2645: f64, t2650: f64, t7478: f64, t7485: f64, t7488: f64, t7491: f64, t7495: f64, t7838: f64, t7846: f64, t7849: f64, t7852: f64, t862: f64, t874: f64, t893: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7857 = t7856 * t7256;
    let t7858 = t7857 * t6534;
    let t7859 = t894 * t7858;
    let t7862 = t2722 * t7354;
    let t7865 = t896 * t2263;
    let t7866 = t7865 * t6534;
    let t7867 = t894 * t7866;
    let t7870 = t2595 * t7298;
    let t7871 = t7870 * t6534;
    let t7872 = t894 * t7871;
    let t7877 = 0.71000632978163088351e-1_f64 * t2640 * t7478 + 0.11833438829693848058e0_f64 * t2640 * t7485 - 0.75734008510040627576e0_f64 * t7488 * t2645 + 0.18314556960919660338e2_f64 * t7491 * t7495 + 0.35500316489081544176e-1_f64 * t874 * t7838 - 0.28977204965962526182e-1_f64 * t2583 * t2591 - 0.48295341609937543638e-1_f64 * t2583 * t2598 + 0.36221506207453157727e-2_f64 * t7846 + 0.60369177012421929545e-2_f64 * t7849 + 0.18110753103726578864e-2_f64 * t893 * t7852 + 0.80492236016562572728e-2_f64 * t893 * t7859 - t862 * t7862 / 48.0_f64 + 0.10866451862235947318e-1_f64 * t893 * t7867 - 0.18110753103726578864e-1_f64 * t893 * t7872 + 0.57954409931925052365e-1_f64 * t2583 * t2650;
    (t7857, t7858, t7859, t7862, t7865, t7866, t7867, t7870, t7871, t7872, t7877)
}
