//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 874/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk874<F: Float>(t11792: F, t11796: F, t11800: F, t11806: F, t11809: F, t11811: F, t11816: F, t11818: F, t11820: F, t11823: F, t11826: F, t11829: F, t11832: F, t11838: F, t11841: F, t11843: F, t11845: F, t11851: F, t11855: F, t11858: F) -> (F,) {
    let t11860 = 0.17376185052903442709e-3 * t11792 - 0.25745714186718600948e-5 * t11796 - 0.10860115658064651693e-4 * t11800 + 0.12290803273518880209e-8 * t11806 - 0.17585468531730730268e-8 * t11809 + 0.16573913624765925007e-7 * t11811 + 0.61454016367594401047e-9 * t11816 - 0.16021449837273727507e-6 * t11818 + 0.5627349930153833686e-7 * t11820 - 0.15176747947735985782e-6 * t11823 + 0.26984257851074582722e-6 * t11826 - 0.61454016367594401047e-9 * t11829 + 0.40969344245062934031e-9 * t11832 - 0.55246378749219750023e-8 * t11838 - 0.28985453471303521737e-5 * t11841 + 0.57970906942607043474e-5 * t11843 - 0.57970906942607043474e-5 * t11845 + 0.14480154210752868924e-5 * t11851 - 0.12670134934408760309e-3 * t11855 - 0.63252766927083333336e-6 * t11858;
    (t11860,)
}
