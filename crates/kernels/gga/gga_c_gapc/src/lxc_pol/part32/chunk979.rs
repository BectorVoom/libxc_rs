//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 979/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk979<F: Float>(t11792: F, t11796: F, t11800: F, t11806: F, t11809: F, t11811: F, t11816: F, t11818: F, t11820: F, t11823: F, t11826: F, t11829: F, t11832: F, t11838: F, t11841: F, t11843: F, t11845: F, t11851: F, t11855: F, t11858: F) -> F {
    let t11860 = F::cast_from(0.17376185052903442709e-3_f64) * t11792 - F::cast_from(0.25745714186718600948e-5_f64) * t11796 - F::cast_from(0.10860115658064651693e-4_f64) * t11800 + F::cast_from(0.12290803273518880209e-8_f64) * t11806 - F::cast_from(0.17585468531730730268e-8_f64) * t11809 + F::cast_from(0.16573913624765925007e-7_f64) * t11811 + F::cast_from(0.61454016367594401047e-9_f64) * t11816 - F::cast_from(0.16021449837273727507e-6_f64) * t11818 + F::cast_from(0.5627349930153833686e-7_f64) * t11820 - F::cast_from(0.15176747947735985782e-6_f64) * t11823 + F::cast_from(0.26984257851074582722e-6_f64) * t11826 - F::cast_from(0.61454016367594401047e-9_f64) * t11829 + F::cast_from(0.40969344245062934031e-9_f64) * t11832 - F::cast_from(0.55246378749219750023e-8_f64) * t11838 - F::cast_from(0.28985453471303521737e-5_f64) * t11841 + F::cast_from(0.57970906942607043474e-5_f64) * t11843 - F::cast_from(0.57970906942607043474e-5_f64) * t11845 + F::cast_from(0.14480154210752868924e-5_f64) * t11851 - F::cast_from(0.12670134934408760309e-3_f64) * t11855 - F::cast_from(0.63252766927083333336e-6_f64) * t11858;
    t11860
}
