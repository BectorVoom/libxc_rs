//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 796/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk796<F: Float>(t2058: F, t542: F, t133: F, t139: F, t140: F, t2036: F, t23725: F, t23745: F, t23810: F, t23866: F, t32140: F, t32164: F, t32175: F, t32187: F, t32281: F, t32308: F, t32752: F, t32756: F, t32764: F, t32767: F, t32768: F, t32774: F, t32775: F, t32782: F, t32786: F, t32791: F, t32797: F, t32803: F, t543: F, t5821: F, t5824: F, t7335: F, t8852: F, t8859: F) -> (F, F, F) {
    let t32806 = t542 * t2058;
    let t32809 = t133 * t2058;
    let t32812 = F::cast_from(0.41054213886971219988e0_f64) * t23810 * t32752 - F::cast_from(0.20527106943485609994e0_f64) * t8852 * t32756 - F::cast_from(0.82108427773942439976e0_f64) * t23866 * t32752 + F::cast_from(0.41054213886971219988e0_f64) * t8859 * t32756 + F::cast_from(0.26675734978222673832e-1_f64) * t32764 * t32281 + F::cast_from(0.80027204934668021496e-1_f64) * t32767 * t32140 * t32768 - F::cast_from(0.12004080740200203224e0_f64) * t32774 * t32140 * t32775 + F::cast_from(0.72503285312204600893e0_f64) * t5821 * t32175 - t32782 - F::cast_from(0.72503285312204600893e0_f64) * t5824 * t32175 + t32786 - F::cast_from(0.21340587982578139066e0_f64) * t7335 * t32308 - F::cast_from(0.12081826776807659559e1_f64) * t543 * t32187 - F::cast_from(0.13684737962323739996e1_f64) * t2036 * t32791 * t139 + F::cast_from(0.10594292039522084817e-1_f64) * t32797 * t23725 + F::cast_from(0.12081826776807659559e1_f64) * t140 * t32187 + F::cast_from(0.22653425206514361674e0_f64) * t23745 * t32803 - F::cast_from(0.29389470585448002138e-1_f64) * t32806 * t32164 + F::cast_from(0.29389470585448002138e-1_f64) * t32809 * t32164;
    (t32806, t32809, t32812)
}
