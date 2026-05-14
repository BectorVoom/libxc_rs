//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1304/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1304<F: Float>(t1008: F, t5555: F, t554: F, t93178: F, t105019: F, t22591: F, t538: F, t22711: F, t23676: F, t100526: F, t101031: F, t105021: F, t105061: F, t23831: F, t23847: F, t23869: F, t26692: F, t3356: F, t40087: F, t8833: F, t94716: F, t94719: F, t94722: F, t94754: F, t94771: F, t94785: F, t94788: F, t94891: F, t94892: F) -> (F, F, F) {
    let t105157 = t5555 * t1008;
    let t105159 = t93178 * t105157 * t554;
    let t105163 = t22591 * t105019 * t538;
    let t105167 = t23676 * t22711 * t1008;
    let t105182 = 0.46992870109762241322e0 * t94716 - 0.10001700163888888889e0 * t94719 - 0.44452000728395061731e-1 * t94722 - 0.1611184118048991131e0 * t94754 - 0.94171484795751865041e-2 * t94771 + 0.10947790369858991997e1 * t94891 * t94892 * t3356 - 0.43791161479435967988e1 * t40087 * t105159 - 0.90613700826057446696e0 * t23847 * t105163 + 0.24441738422985905191e0 * t23831 * t105167 - 0.45306850413028723348e0 * t23847 * t105061 + 0.33339000546296296298e-1 * t26692 * t100526 - 0.90613700826057446696e0 * t8833 * t105021 + 0.90613700826057446696e0 * t23869 * t105163 + 0.18834296959150373008e-1 * t94785 * t101031 - 0.94171484795751865041e-2 * t94788 * t101031;
    (t105159, t105167, t105182)
}
