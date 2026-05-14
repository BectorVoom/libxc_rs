//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 900/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk900<F: Float>(t2059: F, t72: F, t22643: F, t5821: F, t140: F, t2036: F, t22610: F, t22629: F, t22639: F, t22673: F, t22767: F, t23742: F, t23745: F, t23755: F, t23758: F, t23766: F, t23770: F, t23774: F, t539: F, t555: F, t5579: F, t5790: F, t5813: F, t5824: F, t5829: F, t5830: F, t5838: F, t8812: F) -> (F, F, F) {
    let t23775 = t72 * t2059;
    let t23781 = t5821 * t22643;
    let t23783 = 0.33339000546296296298e-1 * t5838 * t22610 - 0.76518236253115177207e1 * t140 * t22673 + 0.56502890877451119026e-1 * t23742 * t22629 - 0.28251445438725559513e-1 * t23745 * t22629 - 0.10947790369858991998e1 * t8812 * t5790 * t539 + 0.54738951849294959988e0 * t2036 * t5790 * t555 - 0.10001700163888888889e0 * t5813 * t23755 + 0.10001700163888888889e0 * t5829 * t5579 * t23758 - 0.53342400874074074075e0 * t5829 * t22767 * t5830 + 0.66678001092592592595e-1 * t23766 + 0.14097861032928672397e1 * t5824 * t22639 - 0.80559205902449556552e-1 * t23770 - 0.30005100491666666667e0 * t23774 * t5579 * t23775 - 0.14097861032928672397e1 * t5821 * t22639 + 0.80559205902449556552e-1 * t23781;
    (t23775, t23781, t23783)
}
