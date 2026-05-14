//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 841/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk841<F: Float>(t2024: F, t30344: F, t30400: F, t35478: F, t35481: F, t35484: F, t35487: F, t35497: F, t39841: F, t39842: F, t39874: F, t45825: F, t45827: F, t45830: F, t45832: F, t45836: F, t45844: F, t45846: F, t5016: F, t739: F, t9840: F) -> (F,) {
    let t45854 = 0.85129199786595678796e-5 * t45825 + 0.1064114997332445985e-4 * t45827 - t39841 + 0.59590439850616975157e-4 * t39842 + 0.6818665413561335432e-1 * t45830 + 0.68186654135613354322e-2 * t45832 - 0.51077519871957407276e-4 * t45836 + 0.23948483403727617128e0 * t739 * t2024 * t30344 + 0.23948483403727617128e0 * t739 * t2024 * t30400 + t39874 - 0.85129199786595678796e-5 * t45844 - 0.42564599893297839398e-5 * t45846 - 0.11974241701863808564e0 * t5016 * t9840 + 0.81300399444200075504e-3 * t35478 - 0.1951603679568577289e-3 * t35481 + 0.81300399444200075504e-3 * t35484 - 0.1951603679568577289e-3 * t35487 + t35497;
    (t45854,)
}
