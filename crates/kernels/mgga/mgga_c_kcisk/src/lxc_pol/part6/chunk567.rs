//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 567/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk567<F: Float>(t1433: F, t7897: F, t457: F, t2110: F, t1421: F, t338: F, t3519: F, t456: F, t5893: F, t5918: F, t5941: F, t7828: F, t7846: F, t7850: F, t7854: F, t7858: F, t7862: F, t7866: F, t7870: F, t7874: F, t7879: F) -> (F, F, F) {
    let t7898 = t1433 * t7897;
    let t7899 = t457 * t7898;
    let t7902 = t2110 * t2110;
    let t7906 = -t3519 + F::new(0.8760572888888888889e-3) * t5893 + F::new(0.19711289e-2) * t5918 - F::new(0.13140859333333333333e-2) * t5941 + F::new(0.10950716111111111111e-2) * t1421 * t7846 + F::new(0.19711289e-2) * t1421 * t7850 - F::new(0.13140859333333333333e-2) * t1421 * t7854 - F::new(0.13140859333333333333e-2) * t1421 * t7858 + F::new(0.65704296666666666667e-3) * t1421 * t7862 + F::new(0.7391733375e-3) * t456 * t7866 - F::new(0.295669335e-2) * t1421 * t7870 + F::new(0.1478346675e-2) * t456 * t7874 + F::new(0.19711289e-2) * t456 * t7879 - F::new(0.98556445e-3) * t456 * t7899 - F::new(4.0) * t7902 - F::new(4.0) * t338 * t7828;
    (t7898, t7899, t7906)
}
