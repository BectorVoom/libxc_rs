//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1301/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1301<F: Float>(t33701: F, t33704: F, t33707: F, t33710: F, t33714: F, t33717: F, t33719: F, t33726: F, t33728: F, t33731: F, t33734: F, t33770: F, t33772: F, t33774: F, t33777: F, t33779: F, t33784: F, t33787: F, t33789: F, t33791: F, t33793: F, t33796: F) -> (F, F) {
    let t37848 = -F::cast_from(0.86880925264517213544e-4_f64) * t33701 - F::cast_from(0.86880925264517213544e-4_f64) * t33704 - F::cast_from(0.43440462632258606772e-4_f64) * t33707 - F::cast_from(0.8244751209615223961e-5_f64) * t33710 + F::cast_from(0.14659167650695868203e-4_f64) * t33714 - F::cast_from(0.14748963928222656251e-7_f64) * t33717 + F::cast_from(0.25301106770833333335e-5_f64) * t33719 + F::cast_from(0.8096354166666666667e-4_f64) * t33726 - F::cast_from(0.22745373045674261828e-5_f64) * t33728 - F::cast_from(0.22745373045674261828e-5_f64) * t33731 + F::cast_from(0.9275345110817126956e-4_f64) * t33734;
    let t37875 = -F::cast_from(0.21135226489492151266e-6_f64) * t33770 - F::cast_from(0.8004342540650813035e-7_f64) * t33772 - F::cast_from(0.80189736504692130024e-6_f64) * t33774 + F::cast_from(0.13259130899812740005e-6_f64) * t33777 - F::cast_from(0.11594181388521408695e-4_f64) * t33779 - F::cast_from(0.38672465124453825014e-8_f64) * t33784 + F::cast_from(0.5691280480400994668e-7_f64) * t33787 + F::cast_from(0.6956508833112845217e-4_f64) * t33789 - F::cast_from(0.84410248952307505288e-7_f64) * t33791 + F::cast_from(0.67528199161846004231e-6_f64) * t33793 + F::cast_from(0.42205124476153752644e-7_f64) * t33796;
    (t37848, t37875)
}
