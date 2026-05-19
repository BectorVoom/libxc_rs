//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1113/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1113<F: Float>(t3379: F, t52: F, t7182: F, t105260: F, t12411: F, t136847: F, t139: F, t145071: F, t147344: F, t147474: F, t147497: F, t147505: F, t147511: F, t147517: F, t147521: F, t2036: F, t23701: F, t23711: F, t23742: F, t23745: F, t23810: F, t23866: F, t32767: F, t32822: F, t3405: F, t34864: F, t34884: F, t34906: F, t39852: F, t7318: F, t8859: F) -> (F, F) {
    let t147533 = t52 * t7182 * t3379;
    let t147541 = -F::cast_from(0.82108427773942439976e0_f64) * t105260 * t34864 - F::cast_from(0.82108427773942439976e0_f64) * t23866 * t147497 - F::cast_from(0.54738951849294959985e1_f64) * t8859 * t147474 + F::cast_from(0.41054213886971219988e0_f64) * t23810 * t147497 - F::cast_from(0.80027204934668021493e-1_f64) * t147505 - F::cast_from(0.42681175965156278131e0_f64) * t32767 * t136847 * t34906 + F::cast_from(0.53351469956445347664e-1_f64) * t147511 - F::cast_from(0.10263553471742804997e0_f64) * t2036 * t7318 * t3405 + F::cast_from(0.12081826776807659559e1_f64) * t23742 * t147517 + F::cast_from(0.22653425206514361674e0_f64) * t23745 * t147521 - F::cast_from(0.22653425206514361674e0_f64) * t23742 * t147521 - F::cast_from(0.12081826776807659559e1_f64) * t23745 * t147517 - F::cast_from(0.82108427773942439976e0_f64) * t39852 * t147344 + F::cast_from(0.24167761770734866964e0_f64) * t23711 * t145071 + F::cast_from(0.45306850413028723348e0_f64) * t32822 * t147533 + F::cast_from(0.45306850413028723348e0_f64) * t12411 * t139 * t34884 - F::cast_from(0.24167761770734866964e0_f64) * t23701 * t145071;
    (t147533, t147541)
}
