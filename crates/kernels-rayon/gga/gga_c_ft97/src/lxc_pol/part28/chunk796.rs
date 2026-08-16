//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 796/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk796(t2058: f64, t542: f64, t133: f64, t139: f64, t140: f64, t2036: f64, t23725: f64, t23745: f64, t23810: f64, t23866: f64, t32140: f64, t32164: f64, t32175: f64, t32187: f64, t32281: f64, t32308: f64, t32752: f64, t32756: f64, t32764: f64, t32767: f64, t32768: f64, t32774: f64, t32775: f64, t32782: f64, t32786: f64, t32791: f64, t32797: f64, t32803: f64, t543: f64, t5821: f64, t5824: f64, t7335: f64, t8852: f64, t8859: f64) -> (f64, f64, f64) {
    let t32806 = t542 * t2058;
    let t32809 = t133 * t2058;
    let t32812 = 0.41054213886971219988e0_f64 * t23810 * t32752 - 0.20527106943485609994e0_f64 * t8852 * t32756 - 0.82108427773942439976e0_f64 * t23866 * t32752 + 0.41054213886971219988e0_f64 * t8859 * t32756 + 0.26675734978222673832e-1_f64 * t32764 * t32281 + 0.80027204934668021496e-1_f64 * t32767 * t32140 * t32768 - 0.12004080740200203224e0_f64 * t32774 * t32140 * t32775 + 0.72503285312204600893e0_f64 * t5821 * t32175 - t32782 - 0.72503285312204600893e0_f64 * t5824 * t32175 + t32786 - 0.21340587982578139066e0_f64 * t7335 * t32308 - 0.12081826776807659559e1_f64 * t543 * t32187 - 0.13684737962323739996e1_f64 * t2036 * t32791 * t139 + 0.10594292039522084817e-1_f64 * t32797 * t23725 + 0.12081826776807659559e1_f64 * t140 * t32187 + 0.22653425206514361674e0_f64 * t23745 * t32803 - 0.29389470585448002138e-1_f64 * t32806 * t32164 + 0.29389470585448002138e-1_f64 * t32809 * t32164;
    (t32806, t32809, t32812)
}
